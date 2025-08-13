use super::{serial_commands::SerialCommand, serial_responses::SerialResponse};
use serialport::{self, SerialPort};
use std::{
    collections::HashMap,
    time::{self},
};

#[derive(Debug)]
pub struct SerialCommunicator {
    hash_map: HashMap<String, Box<dyn SerialPort>>,
    baud_rate: u32,
    message_length: usize,
    timeout: u64,
}

impl SerialCommunicator {
    pub fn new() -> SerialCommunicator {
        SerialCommunicator {
            hash_map: HashMap::new(),
            baud_rate: 115200,
            message_length: 16,
            timeout: 4000,
        }
    }

    pub fn send_command(&mut self, tty_path: &String, command: &SerialCommand) -> SerialResponse {
        self.send(tty_path, &command.serialize());
        self.receive(tty_path)
    }

    pub fn send_command_without_response(&mut self, tty_path: &String, command: &SerialCommand) {
        self.send(tty_path, &command.serialize());
    }

    fn send(&mut self, tty_path: &String, data: &Vec<u8>) {
        if !self.hash_map.contains_key(tty_path) {
            let port = serialport::new(tty_path, self.baud_rate)
                .timeout(time::Duration::from_secs(self.timeout))
                .open()
                .expect("Failed to open port.");

            self.hash_map.insert(tty_path.to_owned(), port);
        }

        self.hash_map
            .get_mut(tty_path)
            .unwrap()
            .clear(serialport::ClearBuffer::All)
            .expect("Could not clear buffer.");

        self.hash_map
            .get_mut(tty_path)
            .unwrap()
            .write_all(data)
            .expect("Failed to write message to port.");

        println!("Sending to port {} data {:?}", tty_path, data);

        let _ = self.hash_map.get_mut(tty_path).unwrap().flush();
    }

    fn receive(&mut self, port_name: &String) -> SerialResponse {
        let mut read_buffer: Vec<u8> = vec![0; self.message_length];

        let read_result = self
            .hash_map
            .get_mut(port_name)
            .unwrap()
            .read(&mut read_buffer);

        if let Err(error) = read_result {
            println!("Error when reading from port {:?}: {:?}", port_name, error);
        } else if read_result.is_ok() {
            print!("Received from port {:?}: ", port_name);
            for byte in &read_buffer {
                print!(" {:#x}", byte);
            }
            println!();
        }

        SerialResponse::try_from(read_buffer).expect("Response parsing failure.")
    }
}
