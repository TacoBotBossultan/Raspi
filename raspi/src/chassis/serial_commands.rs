use super::chassis_traits::EngineOrder;

#[derive(Debug, Clone)]
pub struct SetSpeeds {
    front_right_motor: u8,
    front_left_motor: u8,
    back_left_motor: u8,
    back_right_motor: u8,
}
#[derive(Debug, Clone)]
pub struct SetPosition {
    x_coordinate: i32,
    y_coordinate: i32,
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
    BeerMe,
}

impl GiveMePosition {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GiveMePosition {
    fn default() -> Self {
        Self::new()
    }
}

impl SetSpeeds {
    pub fn new(
        front_right_motor_order: EngineOrder,
        front_left_motor_order: EngineOrder,
        back_left_motor_order: EngineOrder,
        back_right_motor_order: EngineOrder,
    ) -> Self {
        let front_right_motor = front_right_motor_order as u8;
        let front_left_motor = front_left_motor_order as u8;
        let back_left_motor = back_left_motor_order as u8;
        let back_right_motor = back_right_motor_order as u8;
        Self {
            front_right_motor,
            front_left_motor,
            back_left_motor,
            back_right_motor,
        }
    }
    pub fn new_tzaran(
        front_right_motor: u8,
        front_left_motor: u8,
        back_left_motor: u8,
        back_right_motor: u8,
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
    pub fn new(x_coordinate: i32, y_coordinate: i32, theta: u16) -> Self {
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
                bytes.push(data.front_right_motor);
                bytes.push(data.front_left_motor);
                bytes.push(data.back_left_motor);
                bytes.push(data.back_right_motor);
            }
            SerialCommand::SetPosition(data) => {
                bytes.push(FirmwareCommandType::SetPosition as u8);
                let x_byte_array = data.x_coordinate.to_le_bytes();
                let y_byte_array = data.y_coordinate.to_le_bytes();
                let theta_byte_array = data.theta.to_le_bytes();

                for byte in x_byte_array {
                    bytes.push(byte);
                }
                for byte in y_byte_array {
                    bytes.push(byte);
                }
                for byte in theta_byte_array {
                    bytes.push(byte);
                }

                // bytes.push(data.x_coordinate.to_le_bytes());
                // bytes.push(data.y_coordinate.to_le_bytes());
                // bytes.push(data.theta.to_le_bytes());
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
            SerialCommand::BeerMe => bytes.push(FirmwareCommandType::BeerMe as u8),
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
    BeerMe = 0x69,
}
