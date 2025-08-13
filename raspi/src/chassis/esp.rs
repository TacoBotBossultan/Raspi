use crate::chassis::{chassis_traits::EngineOrder, serial_communication::SerialCommunicator};

use super::{
    chassis_traits::Position,
    serial_commands::{self, SerialCommand},
    serial_responses::SerialResponse,
};

#[derive(Debug)]
pub struct DrivESP {
    tty_address: String,
    serial_communicator: SerialCommunicator,
}

impl DrivESP {
    pub fn new(tty_address: String) -> Self {
        DrivESP {
            tty_address,
            serial_communicator: SerialCommunicator::new(),
        }
    }

    pub fn get_tty_address(&self) -> String {
        self.tty_address.clone()
    }

    pub fn send_get_position_command(&mut self) -> Result<Position, String> {
        let get_position_command =
            SerialCommand::GiveMePosition(serial_commands::GiveMePosition::new());

        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &get_position_command);

        match serial_response {
            SerialResponse::Yes => return Err("No position found".to_string()),
            SerialResponse::No => return Err("No position found".to_string()),
            SerialResponse::HavePosition(have_position_response) => match Position::create(
                None,
                have_position_response.x,
                have_position_response.y,
                have_position_response.theta,
            ) {
                Ok(position) => return Ok(position),
                Err(e) => return Err(e),
            },
        }
    }

    pub fn send_set_position_command(&mut self, position: Position) {
        let set_position_command = SerialCommand::SetPosition(serial_commands::SetPosition::new(
            position.x_coordinate,
            position.y_coordinate,
            position.theta,
        ));

        self.serial_communicator
            .send_command_without_response(&self.tty_address, &set_position_command);
    }

    pub fn send_set_speeds_command(
        &mut self,
        front_right_motor: EngineOrder,
        front_left_motor: EngineOrder,
        back_left_motor: EngineOrder,
        back_right_motor: EngineOrder,
    ) {
        let set_speeds_command = SerialCommand::SetSpeeds(serial_commands::SetSpeeds::new(
            front_right_motor,
            front_left_motor,
            back_left_motor,
            back_right_motor,
        ));

        self.serial_communicator
            .send_command_without_response(&self.tty_address, &set_speeds_command);
    }
}

#[derive(Debug)]
pub struct UtilitiESP {
    tty_address: String,
    serial_communicator: SerialCommunicator,
}

impl UtilitiESP {
    pub fn new(tty_address: String) -> Self {
        UtilitiESP {
            tty_address,
            serial_communicator: SerialCommunicator::new(),
        }
    }

    pub fn get_tty_address(&self) -> String {
        self.tty_address.clone()
    }

    pub fn send_on_led_command(&mut self) {
        let led_on_command = SerialCommand::OnLED;
        self.serial_communicator
            .send_command_without_response(&self.tty_address, &led_on_command);
    }
    pub fn send_off_led_command(&mut self) {
        let led_off_command = SerialCommand::OffLED;
        self.serial_communicator
            .send_command_without_response(&self.tty_address, &led_off_command);
    }
    pub fn send_btn_pressed_command(&mut self) -> bool {
        let btn_pressed_command = SerialCommand::BtnPressed;
        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &btn_pressed_command);
        if let SerialResponse::Yes = serial_response {
            return true;
        } else {
            return false;
        }
    }
    pub fn send_reached_lane_command(&mut self) -> bool {
        let reached_lane_command = SerialCommand::ReachedLane;
        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &reached_lane_command);
        if let SerialResponse::Yes = serial_response {
            return true;
        } else {
            return false;
        }
    }
    pub fn send_push_rack_command(&mut self) {
        let push_rack_command = SerialCommand::PushRack;
        self.serial_communicator
            .send_command_without_response(&self.tty_address, &push_rack_command);
    }
    pub fn send_pull_rack_command(&mut self) {
        let pull_rack_command = SerialCommand::PullRack;
        self.serial_communicator
            .send_command_without_response(&self.tty_address, &pull_rack_command);
    }
    pub fn send_is_it_in_command(&mut self) -> bool {
        let is_it_in_command = SerialCommand::IsItIn;
        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &is_it_in_command);
        if let SerialResponse::Yes = serial_response {
            return true;
        } else {
            return false;
        }
    }
    pub fn send_is_it_out_command(&mut self) -> bool {
        let is_it_out_command = SerialCommand::IsItOut;
        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &is_it_out_command);
        if let SerialResponse::Yes = serial_response {
            return true;
        } else {
            return false;
        }
    }
    pub fn send_beer_me_command(&mut self) {
        let beer_me_command = SerialCommand::BeerMe;
        self.serial_communicator
            .send_command_without_response(&self.tty_address, &beer_me_command);
    }
}
