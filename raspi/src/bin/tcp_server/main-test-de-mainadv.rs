use std::{error::Error, io::ErrorKind, path::Path, sync::Arc};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{self, mpsc, oneshot, Mutex},
};

use stimulator::{
    chassis::simulated_chassis::SimulatedChassis,
    master_controller::master_controller::{Command, MasterController},
    mission_controller::mission_controller::MissionController,
    navigation_computing::navigation_computer::NavigationComputer,
    utils::{
        logging::AsyncLogger,
        stimulator_config::{
            read_config_from_file, set_motor_efficiencies_from_config, set_motor_efficiency,
            wait_for_confirmation_of_using_config, CONFIG_FILE_PATH, KBRD_READ_TIME,
        },
    },
    Request_Response::requests::Requests,
};

static PRE_APPEND_STR: &str = "[TCP-Server]";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let chassis: Arc<sync::Mutex<SimulatedChassis>> =
        Arc::new(sync::Mutex::new(SimulatedChassis::new()));
    let navigation_computer = Arc::new(NavigationComputer::new());

    let join_handle = navigation_computer.clone().start(Arc::clone(&chassis));

    let stdout_mutex = Arc::new(Mutex::new(io::stdout()));
    let stderr_mutex = Arc::new(Mutex::new(io::stderr()));

    let async_logger = AsyncLogger::new(stdout_mutex, stderr_mutex);

    if Path::new(CONFIG_FILE_PATH).exists() {
        match read_config_from_file() {
            Ok(parsed_config) => {
                println!(
                    "Buun ti-am scos config din {CONFIG_FILE_PATH:#?}, sigur sigur vrei sa-l folosesti pe asta?"
                );
                println!("{parsed_config:#?}");
                match wait_for_confirmation_of_using_config().await {
                    Ok(()) => {
                        set_motor_efficiencies_from_config(&chassis, &parsed_config).await;
                        //TODO: da-i set doar la eficienta de la motoare
                    }
                    Err(()) => {
                        let msg = "Nah vezi ba ca nu pot sa fac altceva decat din configu deja scris deocamdata";
                        println!("{msg}");
                    }
                };
            }

            Err(err) => {
                println!("{err:#?}");
                println!("Din pacate n-ai alte optiuni de simulare gen deocamdata, come back on the next release");
            }
        }
    }

    let (master_controller_command_sender, command_receiver) = mpsc::channel(32);
    let (mission_sender, mission_receiver) = mpsc::channel(8);
    let (status_sender, status_receiver) = mpsc::channel(8);

    let master_controller = MasterController::new(
        command_receiver,
        mission_sender,
        status_receiver,
        async_logger.clone(),
    );

    tokio::spawn(async move {
        master_controller.run().await;
    });

    let mission_controller =
        MissionController::new(mission_receiver, status_sender, async_logger.clone());

    tokio::spawn(async move {
        mission_controller.run().await;
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    async_logger
        .out_print(format!("{PRE_APPEND_STR} server asculta pe 127.0.0.1:8080"))
        .await;

    loop {
        let (stream, addr) = listener.accept().await?;
        async_logger
            .out_print(format!("{PRE_APPEND_STR} Avem o conexiune de la: {addr}"))
            .await;

        let master_controller_command_sender_clone = master_controller_command_sender.clone();
        let logger_clone = async_logger.clone();

        tokio::spawn(async move {
            if let Err(e) =
                handle_connection(stream, master_controller_command_sender_clone, logger_clone)
                    .await
            {
                eprintln!("{PRE_APPEND_STR} EROAREE pe conexiunea de la {addr}: {e}");
            }
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    master_controller_command_sender: mpsc::Sender<Command>,
    async_logger: AsyncLogger,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(0) => return Ok(()), // cand se inchide conexiunea
            Ok(n) => n,
            Err(e) => {
                async_logger
                    .err_print(format!(
                        "{PRE_APPEND_STR} Failed to read from socket; closing connection: {e}"
                    ))
                    .await;
                return Err(e.into());
            }
        };

        let request: Requests = match serde_json::from_slice(&buffer[0..n]) {
            Ok(req) => req,
            Err(e) => {
                async_logger
                    .err_print(format!(
                        "{PRE_APPEND_STR} Failed to deserialize request: {e}"
                    ))
                    .await;
                continue;
            }
        };

        async_logger
            .out_print(format!("{PRE_APPEND_STR} Received request: "))
            .await;
        async_logger.out_print(format!("{request:#?}")).await;
        async_logger.out_print("".to_string()).await;

        let (responder, response_receiver) = oneshot::channel();

        let cmd = Command { request, responder };

        if master_controller_command_sender.send(cmd).await.is_err() {
            async_logger.err_print(format!("{PRE_APPEND_STR} Failed to send command to master controller. It may have shut down.")).await;
            break Ok(());
        }

        match response_receiver.await {
            Ok(response) => {
                let response_json = serde_json::to_string(&response)?;

                async_logger.out_print(format!("{response:#?}")).await;
                async_logger
                    .out_print(format!("{PRE_APPEND_STR} Sent response: "))
                    .await;
                async_logger.out_print(format!("{response_json:#?}")).await;

                stream.write_all(response_json.as_bytes()).await?;
            }
            Err(_) => {
                async_logger
                    .err_print(format!(
                        "{PRE_APPEND_STR} Master controller failed to send a response."
                    ))
                    .await;
                break Ok(());
            }
        }
        async_logger
            .out_print(
                "==================================================================".to_string(),
            )
            .await;
    }
}
