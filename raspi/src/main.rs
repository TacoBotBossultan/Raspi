use std::process::Command as ProcessCommand;
use std::{error::Error, io::stdout, sync::Arc};

use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
};
use evdev::KeyCode as EvDevKeyCode;
use evdev::{AbsoluteAxisCode, Device, EventSummary};
use raspi::{
    chassis::real_chassis::RealChassis,
    map_storage::route_storage::MapStorage,
    master_controller::master_controller::{self, Command, MasterController},
    mission_controller::mission_controller::MissionController,
    navigation_computing::navigation_computer::NavigationComputer,
    request_response::requests::Requests,
    utils::logging::{AsyncLogger, clear_screen_and_return_to_zero},
};
use tokio::sync::watch;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::{Mutex, Notify, mpsc, oneshot},
    time::{Duration, sleep},
};

static PRE_APPEND_STR: &str = "[MAIN]";

const DEADZONE_LOWER: u32 = 1050;
const DEADZONE_UPPER: u32 = 1250;
const ALL_STOP: u32 = 1150;
const IMPOSSIBLE_VALUE: i32 = 6969;
const TOLERANCE: u32 = 25;

pub struct ControllerEvents {
    pub left_motor_bank_value: i32,
    pub right_motor_bank_value: i32,
    pub insert_rack_button_value: i32,
    pub extract_rack_button_value: i32,
    pub beer_me_button_value: i32,
    pub light_led_button_value: i32,
    pub extinguish_led_button_value: i32,
    pub continue_control_button_value: i32,
    pub strafe_left_button_value: i32,
    pub strafe_right_button_value: i32,
    pub should_continue: bool,
}

impl ControllerEvents {
    pub fn new() -> ControllerEvents {
        ControllerEvents {
            left_motor_bank_value: 6969,
            right_motor_bank_value: 6969,
            insert_rack_button_value: 6969,
            extract_rack_button_value: 6969,
            beer_me_button_value: 6969,
            light_led_button_value: 6969,
            extinguish_led_button_value: 6969,
            continue_control_button_value: 6969,
            strafe_left_button_value: 6969,
            strafe_right_button_value: 6969,
            should_continue: true,
        }
    }
}

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
    master_controller_command_sender: &mpsc::Sender<master_controller::Command>,
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

        let cmd = master_controller::Command { request, responder };

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

fn find_controller_path(controller_name: &str) -> Option<String> {
    let output = ProcessCommand::new("cat")
        .arg("/proc/bus/input/devices")
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);
    let index = output.find(controller_name)?;

    let output = &output[index..output.len()];
    let index = output.find("H: Handlers=")?;

    let output = &output[index..output.len()].split('\n').nth(0)?;
    let mut handlers = output.split("H: Handlers=").nth(1)?.split(' ');

    Some(
        handlers
            .find(|handler| handler.contains("event"))?
            .to_string(),
    )
}

async fn loop_until_docked(controller_name: &String, chassis_mutex: Arc<Mutex<RealChassis>>) {
    let mut device_path = None;
    println!("Please pair the controller...");

    while device_path.is_none() {
        device_path = find_controller_path(controller_name);
    }

    let device_path = device_path.unwrap();
    println!("Device path is:{} ", device_path);

    let mut device = Device::open(&device_path);
    let path = "/dev/input/".to_owned() + &device_path;
    while device.is_err() {
        device = Device::open(&path);
    }

    println!("Press (X) to continue...");

    let mut controller = device.unwrap();
    let interval = Duration::from_millis(20);

    'outer: loop {
        for ev in controller.fetch_events().unwrap() {
            match ev.destructure() {
                EventSummary::Key(event, code, value) => match code {
                    EvDevKeyCode::BTN_SOUTH => break 'outer,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    println!("Reading PlayStation Controller inputs...");
    let mut lefty_moving: bool = false;
    let mut righty_moving: bool = false;

    let events = ControllerEvents::new();
    let events_mutex = Arc::new(Mutex::new(events));
    let events_mutex_clone = events_mutex.clone();

    let join_handle = spawn(async move {
        parse_events(events_mutex_clone, controller).await;
    });
    let mut previous_left_motor_value = IMPOSSIBLE_VALUE as u32;
    let mut previous_right_motor_value = IMPOSSIBLE_VALUE as u32;

    loop {
        sleep(interval).await;

        let mut mutex_guard = events_mutex.lock().await;

        let left_motor_value = (*mutex_guard).left_motor_bank_value;
        let right_motor_value = (*mutex_guard).right_motor_bank_value;
        let insert_rack_button_value = (*mutex_guard).insert_rack_button_value;
        let extract_rack_button_value = (*mutex_guard).extract_rack_button_value;
        let beer_me_button_value = (*mutex_guard).beer_me_button_value;
        let light_led_button_value = (*mutex_guard).light_led_button_value;
        let extinguish_led_button_value = (*mutex_guard).extinguish_led_button_value;
        let continue_button_value = (*mutex_guard).continue_control_button_value;
        let strafe_left_button_value = (*mutex_guard).strafe_left_button_value;
        let strafe_right_button_value = (*mutex_guard).strafe_right_button_value;

        (*mutex_guard).left_motor_bank_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).right_motor_bank_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).insert_rack_button_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).extract_rack_button_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).beer_me_button_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).light_led_button_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).extinguish_led_button_value = IMPOSSIBLE_VALUE;
        (*mutex_guard).continue_control_button_value = IMPOSSIBLE_VALUE;

        drop(mutex_guard);

        if strafe_left_button_value != 0 || strafe_right_button_value != 0 {
        } else if left_motor_value != IMPOSSIBLE_VALUE || right_motor_value != IMPOSSIBLE_VALUE {
            let current_value = (left_motor_value as f32 / 255.0 * 2400.0) as u32;

            if current_value < DEADZONE_LOWER || current_value > DEADZONE_UPPER {
                if previous_left_motor_value + TOLERANCE < current_value
                    || previous_left_motor_value - TOLERANCE > current_value
                {
                    previous_left_motor_value = current_value;
                    let command = self
                        .wheels_motor_left
                        .create_move_command(MoveCommandCode::SetMotorSpeedDirectly, current_value);
                    self.serial_communicator
                        .send_command_without_response(&command);
                    lefty_moving = true;
                }
            } else if lefty_moving {
                previous_left_motor_value = ALL_STOP;
                let command = self
                    .wheels_motor_left
                    .create_move_command(MoveCommandCode::SetMotorSpeedDirectly, ALL_STOP);
                self.serial_communicator
                    .send_command_without_response(&command);
                lefty_moving = false;
            }
            let current_value = (right_motor_value as f32 / 255.0 * 2400.0) as u32;
            if current_value < DEADZONE_LOWER || current_value > DEADZONE_UPPER {
                if previous_right_motor_value + TOLERANCE < current_value
                    || previous_right_motor_value - TOLERANCE > current_value
                {
                    previous_right_motor_value = current_value;
                    let command = self
                        .wheels_motor_right
                        .create_move_command(MoveCommandCode::SetMotorSpeedDirectly, current_value);
                    self.serial_communicator
                        .send_command_without_response(&command);
                    righty_moving = true;
                }
            } else if righty_moving {
                previous_right_motor_value = ALL_STOP;
                let command = self
                    .wheels_motor_right
                    .create_move_command(MoveCommandCode::SetMotorSpeedDirectly, ALL_STOP);
                self.serial_communicator
                    .send_command_without_response(&command);
                righty_moving = false;
            }
        }
        sleep(interval).await;

        if insert_rack_button_value == 1 {
            let pump_command = self
                .pump
                .create_pump_command(PumpCommandCode::SuckFluidDirectly, 0);
            self.serial_communicator
                .send_command_without_response(&pump_command);
        }
        sleep(interval).await;

        if extract_rack_button_value == 1 {
            let pump_command = self
                .pump
                .create_pump_command(PumpCommandCode::PumpFluidDirectly, 0);
            self.serial_communicator
                .send_command_without_response(&pump_command);
        }
        sleep(interval).await;

        if beer_me_button_value == 1 {
            let pump_command = self
                .pump
                .create_pump_command(PumpCommandCode::SuckTrashDirectly, 0);
            self.serial_communicator
                .send_command_without_response(&pump_command);
        }
        sleep(interval).await;

        if light_led_button_value == 1 {
            let pump_command = self
                .pump
                .create_pump_command(PumpCommandCode::PumpTrashDirectly, 0);
            self.serial_communicator
                .send_command_without_response(&pump_command);
        }
        sleep(interval).await;

        if extinguish_led_button_value == 1 {
            let pump_command = self
                .pump
                .create_pump_command(PumpCommandCode::PumpTrashDirectly, 0);
            self.serial_communicator
                .send_command_without_response(&pump_command);
        }
        sleep(interval).await;

        if continue_button_value == 1 {
            println!("I am docked, stopping threads.");
            let mut mutex_guard = events_mutex.lock().await;
            (*mutex_guard).should_continue = false;
            drop(mutex_guard);
            join_handle.await.unwrap();
            println!("Threads stopped, returning.");
            return;
        }
    }
}

async fn parse_events(controller_events: Arc<Mutex<ControllerEvents>>, mut controller: Device) {
    'outer: loop {
        for ev in controller.fetch_events().unwrap() {
            let mut mutex_guard = controller_events.lock().await;
            if !(*mutex_guard).should_continue {
                break 'outer;
            }
            match ev.destructure() {
                EventSummary::AbsoluteAxis(_, code, value) => match code {
                    AbsoluteAxisCode::ABS_RY => {
                        (*mutex_guard).right_motor_bank_value = value;
                    }
                    AbsoluteAxisCode::ABS_Y => {
                        (*mutex_guard).left_motor_bank_value = value;
                    }
                    _ => {}
                },
                EventSummary::Key(_, key, value) => match key {
                    EvDevKeyCode::BTN_TR => {
                        (*mutex_guard).extract_rack_button_value = value;
                    }
                    EvDevKeyCode::BTN_TR2 => {
                        (*mutex_guard).insert_rack_button_value = value;
                    }
                    EvDevKeyCode::BTN_TL2 => {
                        (*mutex_guard).beer_me_button_value = value;
                    }
                    EvDevKeyCode::BTN_DPAD_UP => {
                        (*mutex_guard).light_led_button_value = value;
                    }
                    EvDevKeyCode::BTN_DPAD_DOWN => {
                        (*mutex_guard).extinguish_led_button_value = value;
                    }
                    EvDevKeyCode::BTN_SOUTH => {
                        (*mutex_guard).should_continue = value == 1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
