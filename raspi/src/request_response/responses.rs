use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum RobotStates {
    Busy,
    Free,
}

#[derive(Debug, Serialize, Clone)]
pub struct PhotoResponse {
    pub photo_data: Vec<u8>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StateResponse {
    pub state: RobotStates,
}

#[derive(Debug, Serialize, Clone)]
pub struct GeneralResponse {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub enum Responses {
    PhotoResponse(PhotoResponse),
    StateResponse(StateResponse),
    GeneralResponse(GeneralResponse),
}

impl StateResponse {
    pub fn new(robot_state: RobotStates) -> Self {
        Self { state: robot_state }
    }
}

impl PhotoResponse {
    pub fn new(photo_data: Vec<u8>) -> Self {
        Self { photo_data }
    }
}

impl GeneralResponse {
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }
}
