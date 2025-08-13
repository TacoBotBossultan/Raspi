use super::chassis_traits::EngineOrder;

#[derive(Debug, Clone)]
pub struct SetSpeeds {
    front_right_motor: EngineOrder,
    front_left_motor: EngineOrder,
    back_left_motor: EngineOrder,
    back_right_motor: EngineOrder,
}
#[derive(Debug, Clone)]
pub struct SetPosition {
    x_coordinate: u32,
    y_coordinate: u32,
    theta: u16,
}
#[derive(Debug, Clone)]
pub struct GiveMePosition {}

#[derive(Debug, Clone)]
pub enum SerialCommand {
    SetSpeeds(SetSpeeds),
    SetPosition(SetPosition),
    OnLED,
    OffLED,
    BtnPressed,
    ReachedLane,
    PushRack,
    IsItIn,
    IsItOut,
    PullRack,
    GiveMePosition(GiveMePosition),
}

impl GiveMePosition {
    pub fn new() -> Self {
        Self {}
    }
}

impl SetSpeeds {
    pub fn new(
        front_right_motor: EngineOrder,
        front_left_motor: EngineOrder,
        back_left_motor: EngineOrder,
        back_right_motor: EngineOrder,
    ) -> Self {
        Self {
            front_right_motor,
            front_left_motor,
            back_left_motor,
            back_right_motor,
        }
    }
}

impl SetPosition {
    pub fn new(x_coordinate: u32, y_coordinate: u32, theta: u16) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            theta,
        }
    }
}

impl SerialCommand {
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            SerialCommand::SetSpeeds(data) => {
                bytes.push(FirmwareCommandType::SetSpeed as u8);
                bytes.push(data.front_right_motor.clone() as u8);
                bytes.push(data.front_left_motor.clone() as u8);
                bytes.push(data.back_left_motor.clone() as u8);
                bytes.push(data.back_right_motor.clone() as u8);
            }
            SerialCommand::SetPosition(data) => {
                bytes.push(FirmwareCommandType::SetPosition as u8);
                //TODO: sa-l intrebati pe Gâscă daca e big endian sau lil' endian ca nu-mi dau
                //seama daca le pune in ordinea corecta aici
                bytes.extend_from_slice(&data.x_coordinate.to_be_bytes());
                bytes.extend_from_slice(&data.y_coordinate.to_be_bytes());
                bytes.extend_from_slice(&data.theta.to_be_bytes());
            }
            SerialCommand::OnLED => {
                bytes.push(FirmwareCommandType::OnLED as u8);
            }
            SerialCommand::OffLED => {
                bytes.push(FirmwareCommandType::OffLED as u8);
            }
            SerialCommand::BtnPressed => {
                bytes.push(FirmwareCommandType::BtnPressed as u8);
            }
            SerialCommand::ReachedLane => {
                bytes.push(FirmwareCommandType::ReachedLane as u8);
            }
            SerialCommand::PushRack => {
                bytes.push(FirmwareCommandType::PushRack as u8);
            }
            SerialCommand::IsItIn => {
                bytes.push(FirmwareCommandType::IsItIn as u8);
            }
            SerialCommand::PullRack => {
                bytes.push(FirmwareCommandType::PullRack as u8);
            }
            SerialCommand::IsItOut => {
                bytes.push(FirmwareCommandType::IsItOut as u8);
            }
            SerialCommand::GiveMePosition(_) => {
                bytes.push(FirmwareCommandType::GiveMePosition as u8);
            }
        }
        bytes
    }
}

pub enum FirmwareCommandType {
    GiveMePosition = 0x34,
    SetSpeed = 0x35,
    OnLED = 0x36,
    OffLED = 0x37,
    BtnPressed = 0x38,
    ReachedLane = 0x39,
    PushRack = 0x41,
    IsItIn = 0x42,
    PullRack = 0x43,
    IsItOut = 0x44,
    SetPosition = 0x45,
}
