use super::{serial_commands::SerialCommand, serial_responses::SerialResponse};
use serialport::{self, SerialPort, SerialPortBuilder};
use std::{
    collections::HashMap,
    time::{self},
};

#[derive(Debug)]
pub struct SerialCommunicator {
    serial_port: Box<dyn SerialPort>,
    message_length: usize,
}

impl SerialCommunicator {
    pub fn new(serial_port_path: &str) -> SerialCommunicator {
        SerialCommunicator {
            serial_port: serialport::new(serial_port_path, 115200)
                .timeout(time::Duration::from_secs(4000))
                .open()
                .expect("Failed to open port."),
            message_length: 16,
        }
    }

    pub fn send_command(&mut self, command: &SerialCommand) -> SerialResponse {
        self.send(&command.serialize());
        self.receive()
    }

    pub fn send_command_without_response(&mut self, command: &SerialCommand) {
        self.send(&command.serialize());
    }

    fn send(&mut self, data: &Vec<u8>) {
        self.serial_port
            .clear(serialport::ClearBuffer::All)
            .expect("Could not clear buffer.");

        self.serial_port
            .write_all(data)
            .expect("Failed to write message to port.");

        println!("Sending to port {:?} data {:?}", self.serial_port, data);

        let _ = self.serial_port.flush();
    }

    fn receive(&mut self) -> SerialResponse {
        let mut read_buffer: Vec<u8> = vec![0; self.message_length];

        let read_result = self.serial_port.read(&mut read_buffer);

        if let Err(error) = read_result {
            println!(
                "Error when reading from port {:?}: {:?}",
                self.serial_port.name(),
                error
            );
        } else if read_result.is_ok() {
            print!("Received from port {:?}: ", self.serial_port.name());
            for byte in &read_buffer {
                print!(" {:#x}", byte);
            }
            println!();
        }

        SerialResponse::try_from(read_buffer).expect("Response parsing failure.")
    }
}
