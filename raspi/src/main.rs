use std::{error::Error, io::stdout, sync::Arc};

use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
};
use raspi::{
    chassis::real_chassis::RealChassis,
    map_storage::route_storage::MapStorage,
    master_controller::master_controller::{Command, MasterController},
    mission_controller::mission_controller::MissionController,
    navigation_computing::navigation_computer::NavigationComputer,
    request_response::requests::Requests,
    utils::logging::{AsyncLogger, clear_screen_and_return_to_zero},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::{Mutex, Notify, mpsc, oneshot},
    time::{Duration, sleep},
};

static PRE_APPEND_STR: &str = "[MAIN]";

#[tokio::main]
async fn main() {
    let stdout_mutex = Arc::new(Mutex::new(io::stdout()));
    let stderr_mutex = Arc::new(Mutex::new(io::stderr()));
    let async_logger = AsyncLogger::new(stdout_mutex, stderr_mutex);
    let chassis = RealChassis::new();
    let chassis_arc = Arc::new(Mutex::new(chassis));
    let nav_computer = NavigationComputer::new();
    let mission_controller = MissionController::new(async_logger.clone());
    let (master_controller_command_sender, command_receiver) = mpsc::channel(32);
    let map_storage = MapStorage::new();
    let master_control = MasterController::new();
    let master_arc = Arc::new(master_control);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    async_logger
        .out_print(format!("{PRE_APPEND_STR} server asculta pe 127.0.0.1:8080"))
        .await;

    let (mut stream, addr) = listener.accept().await.unwrap();
    async_logger
        .out_print(format!("{PRE_APPEND_STR} Avem o conexiune de la: {addr}"))
        .await;

    let master_controller_command_sender_clone = master_controller_command_sender.clone();
    let mut logger_clone = async_logger.clone();

    let join_handle = master_arc
        .clone()
        .run(
            mission_controller,
            nav_computer,
            chassis_arc,
            command_receiver,
            map_storage,
            async_logger.clone(),
        )
        .await;

    clear_screen_and_return_to_zero();
    execute!(stdout(), Hide).unwrap();
    println!("Running, press q or x to quit...");
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    let tcp_handle = spawn(async move {
        loop {
            tokio::select! {
                _ = notify_clone.notified() => {
                    println!("Oprim citirea pe TCP.");
                    break;
                }

                result = handle_connection(
                &mut stream,
                &master_controller_command_sender_clone,
                &mut logger_clone) => {
                    match result {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Eroare pe conexiunea de TCP, boss!");
                        }
                    }
                }
            }
        }
    });

    let read_duration = Duration::from_millis(50);
    loop {
        if poll(read_duration).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                if let KeyCode::Char('q') | KeyCode::Char('x') = event.code {
                    clear_screen_and_return_to_zero();
                    println!("Quitting.");
                    notify.notify_one();
                    tcp_handle.await.unwrap();
                    master_arc.stop();
                    join_handle.await.unwrap();
                    sleep(Duration::from_millis(250)).await;
                    clear_screen_and_return_to_zero();
                    execute!(stdout(), Show).unwrap();
                    break;
                }
            }
        }
    }
}

async fn handle_connection(
    stream: &mut TcpStream,
    master_controller_command_sender: &mpsc::Sender<Command>,
    async_logger: &mut AsyncLogger,
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
