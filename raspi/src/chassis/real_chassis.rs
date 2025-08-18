use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
    process::Command,
};

use crate::chassis::{
    config::Config,
    esp::{DrivESP, UtilitiESP},
};

use super::chassis_traits::{
    ChassisTraits, EngineOrder,
    MotorIndex::{self, BackLeft, BackRight, FrontLeft, FrontRight},
    Position,
};

static PRE_APPEND_STR: &str = "[Real-Chassis]";
static CONFIG_PATH: &str = "/home/pi/Raspi_Official/raspi/src/config.toml";
static SCRIPT_PATH: &str = "/home/pi/Raspi_Official/raspi/src/chassis/id_identifier.sh";

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

    fn get_position(&mut self) -> Result<Position, String> {
        self.drivesp.send_get_position_command()
    }

    fn insert_rack(&mut self) {
        self.utiliesp.send_push_rack_command();
    }

    fn retrieve_rack(&mut self) {
        self.utiliesp.send_pull_rack_command();
    }

    fn are_buttons_pressed(&mut self) -> bool {
        self.utiliesp.send_btn_pressed_command()
    }

    fn arrived_at_a_lane(&mut self) -> bool {
        self.utiliesp.send_reached_lane_command()
    }

    fn set_position(&mut self, position: Position) {
        self.drivesp.send_set_position_command(position);
    }

    fn is_rack_inserted(&mut self) -> bool {
        self.utiliesp.send_is_it_in_command()
    }

    fn is_rack_extracted(&mut self) -> bool {
        self.utiliesp.send_is_it_out_command()
    }

    fn beer_me(&mut self) {
        self.utiliesp.send_beer_me_command();
    }

    fn on_led(&mut self) {
        self.utiliesp.send_on_led_command();
    }

    fn off_led(&mut self) {
        self.utiliesp.send_off_led_command();
    }
}

impl RealChassis {
    pub fn set_motor_speeds_tzaran(
        &mut self,
        front_right_motor_speed: u8,
        front_left_motor_speed: u8,
        back_left_motor_speed: u8,
        back_right_motor_speed: u8,
    ) {
        self.drivesp.send_set_speeds_command_tzaran(
            front_right_motor_speed,
            front_left_motor_speed,
            back_left_motor_speed,
            back_right_motor_speed,
        );
    }

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
            drivesp: DrivESP::new(&drive_tty),
        }
    }

    /// returns (utility_tty, drive_tty)
    fn get_esps_ttys() -> (String, String) {
        let output = Command::new("sh")
            .arg("-c")
            .arg(SCRIPT_PATH)
            .output()
            .expect("Failed to execute script");

        let output_str = String::from_utf8_lossy(&output.stdout);

        println!(
            "Current directory: {}",
            std::env::current_dir().unwrap().display()
        );

        println!("OUtput string : {output_str:#?}");
        let mut tty_map = HashMap::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(" - ").collect();
            if parts.len() == 2 {
                tty_map.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
        println!("{PRE_APPEND_STR} {:?}", tty_map);

        let config = Self::get_esps_proccessor_names_from_config(CONFIG_PATH);

        let utilitiesp_proccessor_id: String;
        let drivesp_proccessor_id: String;

        (utilitiesp_proccessor_id, drivesp_proccessor_id) = match config {
            Ok(config) => (
                config.utilitiesp_proccessor_id,
                config.drivesp_proccessor_id,
            ),
            Err(err_msg) => {
                println!("{PRE_APPEND_STR} {}", err_msg);
                (
                    "CV default idk".to_string(),
                    "alt cv default idk".to_string(),
                )
            }
        };

        println!("TTY_mapu : {tty_map:#?}");

        let utility_tty = tty_map
            .get(&utilitiesp_proccessor_id)
            .expect("UtilitiEsp not found")
            .clone();
        let drive_tty = tty_map
            .get(&drivesp_proccessor_id)
            .expect("DrivESP not found")
            .clone();

        println!("{PRE_APPEND_STR} {},{}", utility_tty, drive_tty);

        (utility_tty, drive_tty)
    }

    fn get_esps_proccessor_names_from_config(path: &str) -> Result<Config, String> {
        let path_obj = Path::new(path);
        if path_obj.exists() {
            let contents = match fs::read_to_string(path_obj) {
                Ok(contents) => contents,
                Err(err) => return Err(err.to_string()),
            };
            let config: Config = match toml::from_str(&contents) {
                Ok(conf) => conf,
                Err(err) => return Err(err.to_string()),
            };
            Ok(config)
        } else {
            Err(format!("Nici nu exista {CONFIG_PATH:#?}"))
        }
    }
}

impl Default for RealChassis {
    fn default() -> Self {
        Self::new()
    }
}
