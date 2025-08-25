use std::net::{TcpListener, TcpStream};

const MAP_PORT: u16 = 12346;
const REQUESTS_PORT: u16 = 12347;

pub struct ComputerCommunicator {
    map_stream: TcpStream,
    mission_stream: TcpStream,
}

impl ComputerCommunicator {
    pub fn connect() -> Result<ComputerCommunicator, String> {
        let map_listener = TcpListener::bind(format!("0.0.0.0:{}", MAP_PORT))
            .map_err(|e| format!("Failed to bind map_listener to address: {}", e))?;

        let map_stream = map_listener
            .accept()
            .map_err(|e| format!("Failed to accept connection: {}", e))?
            .0;

        let request_listener = TcpListener::bind(format!("0.0.0.0:{}", REQUESTS_PORT))
            .map_err(|e| format!("Failed to bind request_listener {}", e))?;

        let mission_stream = request_listener
            .accept()
            .map_err(|e| format!("Failed to accept connection {}", e))?
            .0;

        Ok(ComputerCommunicator {
            map_stream,
            mission_stream,
        })
    }
}
