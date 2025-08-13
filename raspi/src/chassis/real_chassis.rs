use std::{collections::HashMap, process::Command};

use crate::chassis::esp::{DrivESP, UtilitiESP};

use super::chassis_traits::{
    ChassisTraits, EngineOrder,
    MotorIndex::{self, BackLeft, BackRight, FrontLeft, FrontRight},
    Position,
};

static PRE_APPEND_STR: &str = "[Real-Chassis]";

#[derive(Debug)]
pub struct RealChassis {
    current_motor_speeds: HashMap<MotorIndex, EngineOrder>,
    utiliesp: UtilitiESP,
    drivesp: DrivESP,
}

impl ChassisTraits for RealChassis {
    fn set_motor_speeds(
        &mut self,
        front_right_motor_speed: EngineOrder,
        front_left_motor_speed: EngineOrder,
        back_left_motor_speed: EngineOrder,
        back_right_motor_speed: EngineOrder,
    ) {
        if front_right_motor_speed != *self.current_motor_speeds.get(&FrontRight).unwrap()
            || front_left_motor_speed != *self.current_motor_speeds.get(&FrontLeft).unwrap()
            || back_left_motor_speed != *self.current_motor_speeds.get(&BackLeft).unwrap()
            || back_right_motor_speed != *self.current_motor_speeds.get(&BackRight).unwrap()
        {
            self.drivesp.send_set_speeds_command(
                front_right_motor_speed,
                front_left_motor_speed,
                back_left_motor_speed,
                back_right_motor_speed,
            );
        }
    }

    fn stop_motors(&mut self) {
        self.set_motor_speeds(
            EngineOrder::Stop,
            EngineOrder::Stop,
            EngineOrder::Stop,
            EngineOrder::Stop,
        );
    }

    fn get_position(&self) -> Position {
        self.drivesp.send_get_position_command()
    }

    fn insert_rack(&self) {
        self.utiliesp.send_push_rack_command();
    }

    fn retrieve_rack(&self) {
        self.utiliesp.send_pull_rack_command();
    }

    fn are_buttons_pressed(&self) -> bool {
        self.utiliesp.send_btn_pressed_command()
    }

    fn arrived_at_a_lane(&self) -> bool {
        self.utiliesp.send_reached_lane_command()
    }

    fn set_position(&self, position: Position) {
        self.drivesp.send_set_position_command(position);
    }

    fn is_rack_inserted(&self) -> bool {
        self.utiliesp.send_is_it_in_command()
    }

    fn is_rack_extracted(&self) -> bool {
        self.utiliesp.send_is_it_out_command()
    }

    fn beer_me(&self) {
        self.utiliesp.send_beer_me_command();
    }

    fn on_led(&self) {
        self.utiliesp.send_on_led_command();
    }

    fn off_led(&self) {
        self.utiliesp.send_off_led_command();
    }
}
impl RealChassis {
    pub fn new() -> Self {
        let mut init_speeds = HashMap::new();
        init_speeds.insert(FrontRight, EngineOrder::Stop);
        init_speeds.insert(FrontLeft, EngineOrder::Stop);
        init_speeds.insert(BackLeft, EngineOrder::Stop);
        init_speeds.insert(BackRight, EngineOrder::Stop);

        let (utility_tty, drive_tty) = Self::get_esps_ttys();

        Self {
            current_motor_speeds: init_speeds,
            utiliesp: UtilitiESP::new(utility_tty),
            drivesp: DrivESP::new(drive_tty),
        }
    }

    /// returns (utility_tty : String, drive_tty : String )
    fn get_esps_ttys() -> (String, String) {
        //TODO: script care gaseste TTy-u  si seteaza esp-urile
        let output = Command::new("sh")
            .arg("-c")
            .arg("./id_identifier.sh")
            .output()
            .expect("Failed to execute script");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut tty_map = HashMap::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(" - ").collect();
            if parts.len() == 2 {
                tty_map.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
        println!("{:?}", tty_map);

        let utility_tty = tty_map
            .get("utilityeps_or_smth idk")
            .expect("UtilitiEsp not found")
            .clone();
        let drive_tty = tty_map
            .get("driveesp_or_smth idk")
            .expect("DrivESP not found")
            .clone();

        println!("{},{}", utility_tty, drive_tty);

        (utility_tty, drive_tty)
    }
}

impl Default for RealChassis {
    fn default() -> Self {
        Self::new()
    }
}
