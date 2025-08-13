use super::{
    connecting_state::ConnectingState, controller_events::ControllerEvents,
    docked_at_instrument_state::DockedAtInstrumentState,
    docked_at_station_state::DockedAtStationState,
};
use crate::{
    commands::{MoveCommandCode, PumpCommandCode},
    motors::{Pump, WheelMotor},
    paths::InstrumentPathsMapping,
    responses::Response,
    serial_communicator::SerialCommunicator,
};
use evdev::{AbsoluteAxisType, Device, InputEventKind, Key};
use std::process::Command;
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const DEADZONE_LOWER: u32 = 1050;
const DEADZONE_UPPER: u32 = 1250;
const ALL_STOP: u32 = 1150;
const IMPOSSIBLE_VALUE: i32 = 6969;
const TOLERANCE: u32 = 25;

pub struct ControllerEvents {
    pub left_motor_value: i32,
    pub right_motor_value: i32,
    pub suck_fluid_button_value: i32,
    pub pump_fluid_button_value: i32,
    pub suck_trash_button_value: i32,
    pub pump_trash_button_value: i32,
    pub triangle_button_value: i32,
    pub should_continue_reading: bool,
}

impl ControllerEvents {
    pub fn new() -> ControllerEvents {
        ControllerEvents {
            left_motor_value: 6969,
            right_motor_value: 6969,
            suck_fluid_button_value: 6969,
            pump_fluid_button_value: 6969,
            suck_trash_button_value: 6969,
            pump_trash_button_value: 6969,
            triangle_button_value: 6969,
            should_continue_reading: true,
        }
    }
}
pub struct MirelState {
    pub wheels_motor_left: WheelMotor,
    pub wheels_motor_right: WheelMotor,
    pub pump: Pump,
    pub serial_communicator: SerialCommunicator,
    pub paths: Option<InstrumentPathsMapping>,
}

impl MirelState {
    pub fn start_manual_control(mut self, controller_name: &String) -> ConnectingState {
        self.loop_until_docked(controller_name);
        println!("Moving from Mirel to ConnectingState.");
        return ConnectingState {
            wheels_motor_left: self.wheels_motor_left,
            wheels_motor_right: self.wheels_motor_right,
            pump: self.pump,
            serial_communicator: self.serial_communicator,
        };
    }

    pub fn get_back_home(mut self, controller_name: &String) -> DockedAtStationState {
        self.loop_until_docked(controller_name);
        return DockedAtStationState {
            wheels_motor_left: self.wheels_motor_left,
            wheels_motor_right: self.wheels_motor_right,
            pump: self.pump,
            serial_communicator: self.serial_communicator,
            current_instrument_ip: None,
            instrument_paths_mapping: self.paths.unwrap(),
        };
    }

    pub fn get_to_the_chopper(
        mut self,
        current_instrument_ip: &Ipv4Addr,
        controller_name: &String,
    ) -> DockedAtInstrumentState {
        self.loop_until_docked(controller_name);
        return DockedAtInstrumentState {
            wheels_motor_left: self.wheels_motor_left,
            wheels_motor_right: self.wheels_motor_right,
            pump: self.pump,
            instrument_paths_mapping: self.paths.unwrap(),
            serial_communicator: self.serial_communicator,
            current_instrument_ip: Some(*current_instrument_ip),
        };
    }

    fn find_controller_path(controller_name: &str) -> Option<String> {
        let output = Command::new("cat")
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

    fn loop_until_docked(&mut self, controller_name: &String) {
        let mut device_path = None;
        println!("Please pair the controller...");

        while device_path.is_none() {
            device_path = Self::find_controller_path(controller_name);
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
                match ev.kind() {
                    InputEventKind::Key(key) => match key {
                        Key::BTN_SOUTH => break 'outer,
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

        let thread_handle = thread::spawn(|| Self::parse_events(events_mutex_clone, controller));
        let mut previous_left_motor_value = IMPOSSIBLE_VALUE as u32;
        let mut previous_right_motor_value = IMPOSSIBLE_VALUE as u32;

        loop {
            thread::sleep(interval);

            let mut mutex_guard = events_mutex.lock().unwrap();

            let left_motor_value = (*mutex_guard).left_motor_value;
            let right_motor_value = (*mutex_guard).right_motor_value;
            let suck_fluid_button_value = (*mutex_guard).suck_fluid_button_value;
            let pump_fluid_button_value = (*mutex_guard).pump_fluid_button_value;
            let suck_trash_button_value = (*mutex_guard).suck_trash_button_value;
            let pump_trash_button_value = (*mutex_guard).pump_trash_button_value;
            let triangle_button_value = (*mutex_guard).triangle_button_value;

            (*mutex_guard).left_motor_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).right_motor_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).suck_fluid_button_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).pump_fluid_button_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).suck_trash_button_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).pump_trash_button_value = IMPOSSIBLE_VALUE;
            (*mutex_guard).triangle_button_value = IMPOSSIBLE_VALUE;

            drop(mutex_guard);

            if left_motor_value != IMPOSSIBLE_VALUE {
                let current_value = (left_motor_value as f32 / 255.0 * 2400.0) as u32;

                if current_value < DEADZONE_LOWER || current_value > DEADZONE_UPPER {
                    if previous_left_motor_value + TOLERANCE < current_value
                        || previous_left_motor_value - TOLERANCE > current_value
                    {
                        previous_left_motor_value = current_value;
                        let command = self.wheels_motor_left.create_move_command(
                            MoveCommandCode::SetMotorSpeedDirectly,
                            current_value,
                        );
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
            }
            thread::sleep(interval);

            if right_motor_value != IMPOSSIBLE_VALUE {
                let current_value = (right_motor_value as f32 / 255.0 * 2400.0) as u32;
                if current_value < DEADZONE_LOWER || current_value > DEADZONE_UPPER {
                    if previous_right_motor_value + TOLERANCE < current_value
                        || previous_right_motor_value - TOLERANCE > current_value
                    {
                        previous_right_motor_value = current_value;
                        let command = self.wheels_motor_right.create_move_command(
                            MoveCommandCode::SetMotorSpeedDirectly,
                            current_value,
                        );
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
            thread::sleep(interval);

            if suck_fluid_button_value == 0 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::StopSuckingFluidDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            } else if suck_fluid_button_value == 1 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::SuckFluidDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            }
            thread::sleep(interval);

            if pump_fluid_button_value == 0 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::StopPumpingFluidDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            } else if pump_fluid_button_value == 1 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::PumpFluidDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            }
            thread::sleep(interval);

            if suck_trash_button_value == 0 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::StopSuckingTrashDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            } else if suck_trash_button_value == 1 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::SuckTrashDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            }
            thread::sleep(interval);

            if pump_trash_button_value == 0 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::StopPumpingTrashDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            } else if pump_trash_button_value == 1 {
                let pump_command = self
                    .pump
                    .create_pump_command(PumpCommandCode::PumpTrashDirectly, 0);
                self.serial_communicator
                    .send_command_without_response(&pump_command);
            }
            thread::sleep(interval);

            if triangle_button_value == 1 {
                match self.verify_docking() {
                    Ok(_) => {
                        println!("I am docked, stopping threads.");
                        let mut mutex_guard = events_mutex.lock().unwrap();
                        (*mutex_guard).should_continue_reading = false;
                        drop(mutex_guard);
                        thread_handle.join().expect("Could not join threads.");
                        println!("Threads stopped, returning.");
                        return;
                    }
                    Err(_) => (),
                }
            }
        }
    }

    fn verify_docking(&mut self) -> Result<String, String> {
        match self.receive_dock_response() {
            Response::Ok => Ok("Am docked".to_owned()),
            Response::Nok => Err("Not docked yet".to_owned()),
        }
    }

    fn receive_dock_response(&mut self) -> Response {
        self.serial_communicator.send_command(
            &self
                .pump
                .create_pump_command(PumpCommandCode::AreYouDocked, 0),
        )
    }

    fn parse_events(controller_events: Arc<Mutex<ControllerEvents>>, mut controller: Device) {
        'outer: loop {
            for ev in controller.fetch_events().unwrap() {
                let mut mutex_guard = controller_events.lock().unwrap();
                if !(*mutex_guard).should_continue_reading {
                    break 'outer;
                }
                match ev.kind() {
                    InputEventKind::AbsAxis(axis) => match axis {
                        AbsoluteAxisType::ABS_RY => {
                            (*mutex_guard).right_motor_value = ev.value();
                        }
                        AbsoluteAxisType::ABS_Y => {
                            (*mutex_guard).left_motor_value = ev.value();
                        }
                        _ => {}
                    },
                    InputEventKind::Key(key) => match key {
                        Key::BTN_TR => {
                            (*mutex_guard).suck_trash_button_value = ev.value();
                        }
                        Key::BTN_TR2 => {
                            (*mutex_guard).pump_trash_button_value = ev.value();
                        }
                        Key::BTN_TL => {
                            (*mutex_guard).suck_fluid_button_value = ev.value();
                        }
                        Key::BTN_TL2 => {
                            (*mutex_guard).pump_fluid_button_value = ev.value();
                        }
                        Key::BTN_NORTH => {
                            (*mutex_guard).triangle_button_value = ev.value();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
