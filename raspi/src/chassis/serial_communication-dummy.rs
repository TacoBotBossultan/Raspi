// use super::commands::Command;
// use serialport::{self, SerialPort};
// use std::{
//     collections::HashMap,
//     time::{self},
// };
//
// pub struct SerialCommunicator {
//     hash_map: HashMap<String, Box<dyn SerialPort>>,
//     baud_rate: u32,
//     message_length: usize,
//     timeout: u64,
// }
//
// impl SerialCommunicator {
//     pub fn new() -> SerialCommunicator {
//         SerialCommunicator {
//             hash_map: HashMap::new(),
//             baud_rate: 115200,
//             message_length: 16,
//             timeout: 4000,
//         }
//     }
//
//     pub fn send_command(&mut self, command: &impl Command) -> Response {
//         let mut command_data: Vec<u8> = Vec::new();
//         command_data.append(&mut command.serialize());
//         SerialCommunicator::send(self, command.get_serial_port(), &command_data);
//         SerialCommunicator::receive(self, command.get_serial_port())
//     }
//
//     pub fn send_command_without_response(&mut self, command: &impl Command) {
//         let mut command_data: Vec<u8> = Vec::new();
//         command_data.append(&mut command.serialize());
//         command_data.push(0x69);
//         SerialCommunicator::send(self, command.get_serial_port(), &command_data);
//     }
//
//     fn send(&mut self, port_name: &String, data: &Vec<u8>) {
//         if !self.hash_map.contains_key(port_name) {
//             let port = serialport::new(port_name, self.baud_rate)
//                 .timeout(time::Duration::from_secs(self.timeout))
//                 .open()
//                 .expect("Failed to open port.");
//
//             self.hash_map.insert(port_name.to_owned(), port);
//         }
//
//         self.hash_map
//             .get_mut(port_name)
//             .unwrap()
//             .clear(serialport::ClearBuffer::All)
//             .expect("Could not clear buffer.");
//
//         self.hash_map
//             .get_mut(port_name)
//             .unwrap()
//             .write_all(data)
//             .expect("Failed to write message to port.");
//
//         println!("Sending to port {} data {:?}", port_name, data);
//
//         let _ = self.hash_map.get_mut(port_name).unwrap().flush();
//     }
//
//     fn receive(&mut self, port_name: &String) -> Response {
//         let mut read_buffer: Vec<u8> = vec![0; self.message_length];
//
//         let read_result = self
//             .hash_map
//             .get_mut(port_name)
//             .unwrap()
//             .read(&mut read_buffer);
//
//         if let Err(error) = read_result {
//             println!("Error when reading from port {:?}: {:?}", port_name, error);
//         } else if read_result.is_ok() {
//             print!("Received from port {:?}: ", port_name);
//             for byte in &read_buffer {
//                 print!(" {:#x}", byte);
//             }
//             println!();
//         }
//
//         Response::try_from(read_buffer[0]).expect("Response parsing failure.")
//     }
// }
