use super::{serial_commands::SerialCommand, serial_responses::SerialResponse};
use serialport::{self, SerialPort};
use std::time::{self};

#[derive(Debug)]
pub struct SerialCommunicator {
    serial_port: Box<dyn SerialPort>,
    message_length: usize,
}

impl SerialCommunicator {
    pub fn new(serial_port_path: &str) -> SerialCommunicator {
        SerialCommunicator {
            serial_port: serialport::new(serial_port_path, 115200)
                .timeout(time::Duration::from_millis(500))
                .open()
                .unwrap_or_else(|_| panic!("Nu pot sa deschid portu : {serial_port_path:#?}")),
            message_length: 16,
        }
    }

    pub fn send_command(
        &mut self,
        command: &SerialCommand,
    ) -> Result<SerialResponse, &'static str> {
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
        let mut retry_count = 0;
        loop {
            if retry_count >= 3 {
                break;
            }
            retry_count += 1;
            if self.serial_port.write_all(data).is_ok() {
                let _ = self.serial_port.flush();
                break;
            };
        }
    }

    fn receive(&mut self) -> Result<SerialResponse, &'static str> {
        let mut retry_count = 0;
        loop {
            let mut read_buffer: Vec<u8> = vec![0; self.message_length];
            if retry_count >= 3 {
                break;
            }

            let read_result = self.serial_port.read(&mut read_buffer);
            retry_count += 1;
            match read_result {
                Ok(_) => {
                    let parsed_response = SerialResponse::try_from(read_buffer);
                    match parsed_response {
                        Ok(response) => return Ok(response),
                        Err(error) => {
                            println!(
                                "Could not parse a response when reading from port {:?}: {:?}",
                                self.serial_port.name(),
                                error
                            );
                        }
                    }
                }
                Err(error) => {
                    println!(
                        "Error when reading from port {:?}: {:?}",
                        self.serial_port.name(),
                        error
                    );
                }
            }
        }
        Err("Could not receive a response.")
    }
}
