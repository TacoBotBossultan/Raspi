use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::Clear;
use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
};
use evdev::KeyCode as EvDevKeyCode;
use evdev::{AbsoluteAxisCode, Device, EventSummary};
use raspi::chassis::chassis_traits::{ChassisTraits, Position};
use raspi::{
    chassis::real_chassis::RealChassis,
    map_storage::route_storage::MapStorage,
    master_controller::master_controller::{self, MasterController},
    mission_controller::mission_controller::MissionController,
    navigation_computing::navigation_computer::NavigationComputer,
    request_response::requests::Requests,
    utils::logging::{AsyncLogger, clear_screen_and_return_to_zero},
};
use std::process::Command as ProcessCommand;
use std::{error::Error, io::stdout, sync::Arc};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::{Mutex, Notify, mpsc, oneshot},
    time::{Duration, sleep},
};

static PRE_APPEND_STR: &str = "[MAIN]";
const STRAFE_FORWARD_SPEED: u8 = 110;
const STRAFE_BACKWARD_SPEED: u8 = 90;
const DEAD_SLOW_AHEAD: u8 = 102;
const UNDEAD_SLOW_AHEAD: u8 = 103;
const DEADZONE_LOWER: i32 = 115;
const DEADZONE_UPPER: i32 = 135;
const ALL_STOP: u8 = 100;
const IMPOSSIBLE_VALUE: i32 = 6969;

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
    pub lane_seek_button_value: i32,
    pub quit_after_controller: bool,
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
            lane_seek_button_value: 6969,
            quit_after_controller: false,
            should_continue: true,
        }
    }
}

impl Default for ControllerEvents {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() {
    execute!(stdout(), Hide).unwrap();

    let chassis = RealChassis::new();
    let chassis_arc = Arc::new(Mutex::new(chassis));
    clear_screen_and_return_to_zero();
    if wait_for_controller("Wireless Controller", chassis_arc.clone()).await {
        return;
    }

    let stdout_mutex = Arc::new(Mutex::new(io::stdout()));
    let stderr_mutex = Arc::new(Mutex::new(io::stderr()));
    let async_logger = AsyncLogger::new(stdout_mutex, stderr_mutex);
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
        if poll(read_duration).unwrap()
            && let Event::Key(event) = read().unwrap()
            && let KeyCode::Char('q') | KeyCode::Char('x') = event.code
        {
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

    let output = &output[index..output.len()].split('\n').next()?;
    let mut handlers = output.split("H: Handlers=").nth(1)?.split(' ');

    Some(
        handlers
            .find(|handler| handler.contains("event"))?
            .to_string(),
    )
}

async fn wait_for_controller(controller_name: &str, chassis_mutex: Arc<Mutex<RealChassis>>) -> bool {
    let mut device_path = None;
    println!("Please pair the controller...");

    while device_path.is_none() {
        device_path = find_controller_path(controller_name);
    }

    let device_path = device_path.unwrap();
    println!("Device path is: {} ", device_path);

    let mut device = Device::open(&device_path);
    let path = "/dev/input/".to_owned() + &device_path;
    while device.is_err() {
        device = Device::open(&path);
    }
    println!("Press (X) or triangle to continue...");
    println!("Pressing (X) will continue running the app after you're done with the controller, pressing the triangle button will quit the app when you're done with the controller.");

    let mut controller = device.unwrap();
    let interval = Duration::from_millis(20);
    let events = ControllerEvents::new();
    let events_mutex = Arc::new(Mutex::new(events));
    let events_mutex_clone = events_mutex.clone();
    let events_mutex_cloner = events_mutex.clone();

    'outer: loop {
        sleep(interval).await;
        for ev in controller.fetch_events().unwrap() {
            let EventSummary::Key(_, code, _) = ev.destructure() else {
                continue;
            };
            if code == EvDevKeyCode::BTN_NORTH {
                let mut mutex_guard = events_mutex.lock().await;
                mutex_guard.quit_after_controller = true;
                drop(mutex_guard);
                break 'outer;
            }
            if code == EvDevKeyCode::BTN_SOUTH {
                break 'outer;
            }
        }
    }

    let mut chassis_lock = chassis_mutex.lock().await;
    chassis_lock.set_position(Position::create(None, 69, 69, 69).unwrap());
    drop(chassis_lock);

    clear_screen_and_return_to_zero();
    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(crossterm::terminal::ClearType::CurrentLine)
    )
    .unwrap();
    execute!(stdout(), MoveTo(0, 0), Print("You have control.")).unwrap();

    let chassis_mutex_clone = chassis_mutex.clone();
    let join_handle = spawn(async move {
        parse_events(events_mutex_clone, controller).await;
    });
    let other_join_handle = spawn(async move {
        loop {
            if !events_mutex_cloner.lock().await.should_continue {
                break;
            }
            sleep(Duration::from_millis(100)).await;
            let mut chassis_lock = chassis_mutex_clone.lock().await;
            let position_response = chassis_lock.get_position();
            let position = match position_response {
            Ok(pos) => pos,
            Err(error) => {
                            println!("{:?}", error);
                            continue;
                        }
            };
            drop(chassis_lock);
            execute!(
                stdout(),
                MoveTo(0, 1),
                Clear(crossterm::terminal::ClearType::CurrentLine)
            )
            .unwrap();
            execute!(
                stdout(),
                MoveTo(0, 1),
                Print(format!("Current position is {:?}.", position))
            )
            .unwrap();
        }
    });
    loop {
        sleep(interval).await;
        let mut chassis_lock;
        let mut mutex_guard = events_mutex.lock().await;

        let left_motor_value = mutex_guard.left_motor_bank_value;
        let right_motor_value = mutex_guard.right_motor_bank_value;
        let insert_rack_button_value = mutex_guard.insert_rack_button_value;
        let extract_rack_button_value = mutex_guard.extract_rack_button_value;
        let beer_me_button_value = mutex_guard.beer_me_button_value;
        let light_led_button_value = mutex_guard.light_led_button_value;
        let extinguish_led_button_value = mutex_guard.extinguish_led_button_value;
        let continue_button_value = mutex_guard.continue_control_button_value;
        let strafe_left_button_value = mutex_guard.strafe_left_button_value;
        let strafe_right_button_value = mutex_guard.strafe_right_button_value;
        let lane_seek_button_value = mutex_guard.lane_seek_button_value;

        mutex_guard.insert_rack_button_value = IMPOSSIBLE_VALUE;
        mutex_guard.extract_rack_button_value = IMPOSSIBLE_VALUE;
        mutex_guard.beer_me_button_value = IMPOSSIBLE_VALUE;
        mutex_guard.light_led_button_value = IMPOSSIBLE_VALUE;
        mutex_guard.extinguish_led_button_value = IMPOSSIBLE_VALUE;
        mutex_guard.continue_control_button_value = IMPOSSIBLE_VALUE;

        drop(mutex_guard);
        execute!(
            stdout(),
            MoveTo(0, 7),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        execute!(
            stdout(),
            MoveTo(0, 7),
            Print(format!(
                "LB: {:?} RB: {:?} IR: {:?} ER: {:?} BM: {:?} LL: {:?} EL: {:?} SL: {:?} SR: {:?} CB: {:?}",
               left_motor_value, right_motor_value, insert_rack_button_value, extract_rack_button_value, beer_me_button_value, light_led_button_value, extinguish_led_button_value, strafe_left_button_value, strafe_right_button_value, continue_button_value 
            ))
        )
        .unwrap();
        let mut fr_motor_speed: u8 = ALL_STOP;
        let mut fl_motor_speed: u8 = ALL_STOP;
        let mut bl_motor_speed: u8 = ALL_STOP;
        let mut br_motor_speed: u8 = ALL_STOP;
        if lane_seek_button_value != IMPOSSIBLE_VALUE {
            fr_motor_speed = UNDEAD_SLOW_AHEAD;
            fl_motor_speed = DEAD_SLOW_AHEAD;
            bl_motor_speed = UNDEAD_SLOW_AHEAD;
            br_motor_speed = DEAD_SLOW_AHEAD;
        } else if strafe_left_button_value != IMPOSSIBLE_VALUE {
            fr_motor_speed = STRAFE_FORWARD_SPEED;
            fl_motor_speed = STRAFE_BACKWARD_SPEED;
            bl_motor_speed = STRAFE_FORWARD_SPEED;
            br_motor_speed = STRAFE_BACKWARD_SPEED;
        } else if strafe_right_button_value != IMPOSSIBLE_VALUE {
            fr_motor_speed = STRAFE_BACKWARD_SPEED;
            fl_motor_speed = STRAFE_FORWARD_SPEED;
            bl_motor_speed = STRAFE_BACKWARD_SPEED;
            br_motor_speed = STRAFE_FORWARD_SPEED;
        } else {
            if left_motor_value != IMPOSSIBLE_VALUE && !(DEADZONE_LOWER..=DEADZONE_UPPER).contains(&left_motor_value) { 
                let mut current_left_bank_value = (left_motor_value as f32 / 255.0 * 200.0) as u8;
                current_left_bank_value = 200 - current_left_bank_value;
                if current_left_bank_value < 1 {
                    current_left_bank_value = 1;
                }
                    fl_motor_speed = current_left_bank_value;
                    bl_motor_speed = current_left_bank_value;
            }
            if right_motor_value != IMPOSSIBLE_VALUE && !(DEADZONE_LOWER..=DEADZONE_UPPER).contains(&right_motor_value) {
                let mut current_right_bank_value = (right_motor_value as f32 / 255.0 * 200.0) as u8;
                current_right_bank_value = 200 - current_right_bank_value;
                if current_right_bank_value < 1 {
                    current_right_bank_value = 1;
                }
                    fr_motor_speed = current_right_bank_value;
                    br_motor_speed = current_right_bank_value;
            }            
        }
        chassis_lock = chassis_mutex.lock().await;
        chassis_lock.set_motor_speeds_tzaran(
            fr_motor_speed,
            fl_motor_speed,
            bl_motor_speed,
            br_motor_speed,
        );
        drop(chassis_lock);

        sleep(interval).await;
        execute!(
            stdout(),
            MoveTo(0, 3),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        execute!(
            stdout(),
            MoveTo(0, 3),
            Print(format!(
                "{:?}          {:?}",
                fl_motor_speed, fr_motor_speed
            ))
        )
        .unwrap();
        execute!(
            stdout(),
            MoveTo(0, 5),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        execute!(
            stdout(),
            MoveTo(0, 5),
            Print(format!(
                "{:?}          {:?}",
                bl_motor_speed, br_motor_speed
            ))
        )
        .unwrap();

        if insert_rack_button_value == 1 {
            chassis_lock = chassis_mutex.lock().await;
            chassis_lock.insert_rack();
            drop(chassis_lock);
        }
        sleep(interval).await;

        if extract_rack_button_value == 1 {
            chassis_lock = chassis_mutex.lock().await;
            chassis_lock.retrieve_rack();
            drop(chassis_lock);
        }
        sleep(interval).await;

        if beer_me_button_value == 1 {
            chassis_lock = chassis_mutex.lock().await;
            chassis_lock.beer_me();
            drop(chassis_lock);
        }
        sleep(interval).await;

        if light_led_button_value == 1 {
            chassis_lock = chassis_mutex.lock().await;
            chassis_lock.on_led();
            drop(chassis_lock);
        }
        sleep(interval).await;

        if extinguish_led_button_value == 1 {
            chassis_lock = chassis_mutex.lock().await;
            chassis_lock.off_led();
            drop(chassis_lock);
        }
        sleep(interval).await;

        if continue_button_value == 1 {
            println!("Stopping threads.");
            let mut mutex_guard = events_mutex.lock().await;
            mutex_guard.should_continue = false;
            let quit_after_controller = mutex_guard.quit_after_controller;
            drop(mutex_guard);
            join_handle.await.unwrap();
            other_join_handle.await.unwrap();
            clear_screen_and_return_to_zero();
            println!("Threads stopped, returning.");
            if quit_after_controller{
                execute!(stdout(), Show).unwrap();
            }
            return quit_after_controller; 
        }
    }
}

async fn parse_events(controller_events: Arc<Mutex<ControllerEvents>>, mut controller: Device) {
    'outer: loop {
        let events = controller.fetch_events().unwrap();
        for ev in events {
            let mut mutex_guard = controller_events.lock().await;
            if !mutex_guard.should_continue {
                drop(mutex_guard);
                break 'outer;
            }
            match ev.destructure() {
                EventSummary::AbsoluteAxis(_, code, value) => match code {
                    AbsoluteAxisCode::ABS_RY => {
                        mutex_guard.right_motor_bank_value = value;
                    }
                    AbsoluteAxisCode::ABS_Y => {
                        mutex_guard.left_motor_bank_value = value;
                    }
                    AbsoluteAxisCode::ABS_HAT0X => {
                        match value {
                            -1 => mutex_guard.strafe_left_button_value = 1,
                            1 => mutex_guard.strafe_right_button_value = 1,
                            0 => {
                                mutex_guard.strafe_left_button_value = IMPOSSIBLE_VALUE;
                                mutex_guard.strafe_right_button_value = IMPOSSIBLE_VALUE;
                            },
                        _ => {}
                        }
                    }
                    AbsoluteAxisCode::ABS_HAT0Y => {
                        match value {
                            -1 => mutex_guard.lane_seek_button_value = 1,
                            0 => {
                                mutex_guard.lane_seek_button_value = IMPOSSIBLE_VALUE;
                            },
                        _ => {}
                        }
                    }
                    _ => {}
                },
                EventSummary::Key(_, key, value) => match key {
                    EvDevKeyCode::BTN_TR => {
                        mutex_guard.extract_rack_button_value = value;
                    }
                    EvDevKeyCode::BTN_TR2 => {
                        mutex_guard.insert_rack_button_value = value;
                    }
                    EvDevKeyCode::BTN_TL2 => {
                        mutex_guard.beer_me_button_value = value;
                    }
                    EvDevKeyCode::BTN_WEST => {
                        mutex_guard.light_led_button_value = value;
                    }
                    EvDevKeyCode::BTN_EAST => {
                        mutex_guard.extinguish_led_button_value = value;
                    }
                    EvDevKeyCode::BTN_NORTH=> {
                        mutex_guard.continue_control_button_value = value;
                    }
                    _ => {}
                },
                _ => {}
            }
            drop(mutex_guard);
        }
    }
}
