use std::u8;

#[derive(PartialEq, Eq)]
pub struct HavePositionResponse {
    pub x: u32,
    pub y: u32,
    pub theta: u16,
}

impl HavePositionResponse {
    pub fn new(x: u32, y: u32, theta: u16) -> Self {
        Self { x, y, theta }
    }
}

#[derive(PartialEq, Eq)]
pub enum SerialResponse {
    Yes,
    No,
    HavePosition(HavePositionResponse),
}

impl TryFrom<Vec<u8>> for SerialResponse {
    type Error = String;

    fn try_from(firmware_response: Vec<u8>) -> Result<Self, Self::Error> {
        match firmware_response[0] {
            val if val == FirmwareResponseType::Yes as u8 => Ok(SerialResponse::Yes),
            val if val == FirmwareResponseType::No as u8 => Ok(SerialResponse::No),
            val if val == FirmwareResponseType::HavePosition as u8 => {
                let x_bytes: [u8; 4] = firmware_response[1..4]
                    .try_into()
                    .expect("Couldn't slice the response to extract the X bytes.");
                let y_bytes: [u8; 4] = firmware_response[5..8]
                    .try_into()
                    .expect("Couldn't slice the response to extract the Y bytes.");
                let theta_bytes: [u8; 2] = firmware_response[9..10]
                    .try_into()
                    .expect("Couldn't slice the response to extract the theta bytes.");
                let x = u32::from_le_bytes(x_bytes);
                let y = u32::from_le_bytes(y_bytes);
                let theta = u16::from_le_bytes(theta_bytes);
                Ok(SerialResponse::HavePosition(HavePositionResponse::new(
                    x, y, theta,
                )))
            }
            res_code => Err(format!("{res_code:?} not a valid response code!")),
        }
    }
}

pub enum FirmwareResponseType {
    Yes = 0x31,
    No = 0x32,
    HavePosition = 0x33,
}

pub enum FirmwareResponse {
    YesResponse,
    NoResponse,
    PositionResponse,
}
