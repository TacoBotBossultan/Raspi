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

    pub fn send_get_position_command(&mut self) -> Position {
        let get_position_command =
            SerialCommand::GiveMePosition(serial_commands::GiveMePosition::new());

        let serial_response = self
            .serial_communicator
            .send_command(&self.tty_address, &get_position_command);

        match serial_response {
            SerialResponse::Yes => todo!(),
            SerialResponse::No => todo!(),
            SerialResponse::HavePosition(have_position_response) => {
                return Position::create(
                    None,
                    have_position_response.x,
                    have_position_response.y,
                    have_position_response.theta,
                )
                .unwrap();
            }
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

    pub fn send_on_led_command(&self) {}
    pub fn send_off_led_command(&self) {}
    pub fn send_btn_pressed_command(&self) -> bool {}
    pub fn send_reached_lane_command(&self) -> bool {}
    pub fn send_push_rack_command(&self) {}
    pub fn send_pull_rack_command(&self) {}
    pub fn send_is_it_in_command(&self) -> bool {}
    pub fn send_is_it_out_command(&self) -> bool {}
    pub fn send_beer_me_command(&self) {}
}
