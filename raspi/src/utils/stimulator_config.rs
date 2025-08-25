use std::{fs, io::stdout, sync::Arc, time::Duration};

use crossterm::{
    cursor::Show,
    event::{Event, KeyCode, poll, read},
    execute,
};
use serde::{Deserialize, Serialize};
use tokio::{sync, time::sleep};

use crate::{
    chassis::{chassis_traits::MotorIndex, simulated_chassis::SimulatedChassis},
    utils::logging::clear_screen_and_return_to_zero,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StartConfig {
    x: i32,
    y: i32,
    orientation: u16,
}

impl StartConfig {
    pub fn new(x: i32, y: i32, orientation: u16) -> Self {
        Self { x, y, orientation }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_orientation(&mut self, orientation: u16) {
        self.orientation = orientation;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn orientation(&self) -> u16 {
        self.orientation
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct TargetConfig {
    x: i32,
    y: i32,
    orientation: u16,
}

impl TargetConfig {
    pub fn new(x: i32, y: i32, orientation: u16) -> Self {
        Self { x, y, orientation }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_orientation(&mut self, orientation: u16) {
        self.orientation = orientation;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn orientation(&self) -> u16 {
        self.orientation
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MotorEfficiencies {
    front_right: u16,
    front_left: u16,
    back_left: u16,
    back_right: u16,
}

impl MotorEfficiencies {
    pub fn new(front_right: u16, front_left: u16, back_left: u16, back_right: u16) -> Self {
        Self {
            front_right,
            front_left,
            back_left,
            back_right,
        }
    }

    pub fn set_front_right(&mut self, front_right: u16) {
        self.front_right = front_right;
    }

    pub fn set_front_left(&mut self, front_left: u16) {
        self.front_left = front_left;
    }

    pub fn set_back_left(&mut self, back_left: u16) {
        self.back_left = back_left;
    }

    pub fn set_back_right(&mut self, back_right: u16) {
        self.back_right = back_right;
    }

    pub fn front_right(&self) -> u16 {
        self.front_right
    }

    pub fn front_left(&self) -> u16 {
        self.front_left
    }

    pub fn back_left(&self) -> u16 {
        self.back_left
    }

    pub fn back_right(&self) -> u16 {
        self.back_right
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Config {
    start: StartConfig,
    target: TargetConfig,
    motor_efficiencies: MotorEfficiencies,
}

impl Config {
    pub fn new(
        start: StartConfig,
        target: TargetConfig,
        motor_efficiencies: MotorEfficiencies,
    ) -> Self {
        Self {
            start,
            target,
            motor_efficiencies,
        }
    }

    pub fn set_start(&mut self, start: StartConfig) {
        self.start = start;
    }

    pub fn set_target(&mut self, target: TargetConfig) {
        self.target = target;
    }

    pub fn set_motor_efficiencies(&mut self, motor_efficiencies: MotorEfficiencies) {
        self.motor_efficiencies = motor_efficiencies;
    }

    pub fn start(&self) -> &StartConfig {
        &self.start
    }

    pub fn target(&self) -> &TargetConfig {
        &self.target
    }

    pub fn motor_efficiencies(&self) -> &MotorEfficiencies {
        &self.motor_efficiencies
    }
}

pub const KBRD_READ_TIME: u64 = 10;
pub const CONFIG_FILE_PATH: &str = "config.toml";

pub fn read_config_from_file() -> Result<Config, String> {
    let config_from_file = fs::read_to_string(CONFIG_FILE_PATH).map_err(|err| {
        let err = format!(
            "N-am putut sa citesc fisieru '{CONFIG_FILE_PATH}': {err}\n 
                    Ma pun sa te intreb direct de la tastaturi"
        );
        err
    })?;

    println!("Buun am citit fisieeru '{CONFIG_FILE_PATH:#?}'. Incerc sa-l si deserializez...");

    let config_from_file: Config = toml::from_str(&config_from_file).map_err(|err| {
        let err = format!(
            "N-am putut sa deserializez '{config_from_file}': {err} \n
               Ma pun sa te intreb direct de la tastaturi "
        );
        err
    })?;

    Ok(config_from_file)
}

pub async fn wait_for_confirmation_of_using_config() -> Result<(), ()> {
    println!("'y/Y' (use this config) / 'n/N' (nah, read anotha one)");
    let read_duration = Duration::from_millis(KBRD_READ_TIME);
    loop {
        if poll(read_duration).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                if let KeyCode::Char('n') = event.code {
                    clear_screen_and_return_to_zero();
                    println!("Aight u the boss ...");
                    sleep(Duration::from_millis(250)).await;
                    clear_screen_and_return_to_zero();
                    execute!(stdout(), Show).unwrap();
                    return Err(());
                }
                if let KeyCode::Char(_) = event.code {
                    clear_screen_and_return_to_zero();
                    println!("Ok putoare...");
                    sleep(Duration::from_millis(250)).await;
                    clear_screen_and_return_to_zero();
                    execute!(stdout(), Show).unwrap();
                    return Ok(());
                }
            }
        }
    }
}

pub fn set_motor_efficiency(
    sim_chassis: &mut SimulatedChassis,
    motor_index: &MotorIndex,
    motor_efficiency: u16,
) -> Result<(), ()> {
    match sim_chassis.set_motor_efficiency(motor_index.clone(), motor_efficiency) {
        Ok(output) => {
            println!("{output:#?}");
            Ok(())
        }
        Err(err_msg) => {
            println!("{err_msg:#?}");
            Err(())
        }
    }
}

pub async fn set_motor_efficiencies_from_config(
    chassis: &Arc<sync::Mutex<SimulatedChassis>>,
    config: &Config,
) {
    let _ = set_motor_efficiency(
        &mut *chassis.lock().await,
        &MotorIndex::FrontRight,
        config.motor_efficiencies().front_right(),
    );
    let _ = set_motor_efficiency(
        &mut *chassis.lock().await,
        &MotorIndex::FrontLeft,
        config.motor_efficiencies().front_left(),
    );
    let _ = set_motor_efficiency(
        &mut *chassis.lock().await,
        &MotorIndex::BackLeft,
        config.motor_efficiencies().back_left(),
    );
    let _ = set_motor_efficiency(
        &mut *chassis.lock().await,
        &MotorIndex::BackRight,
        config.motor_efficiencies().back_right(),
    );
}
