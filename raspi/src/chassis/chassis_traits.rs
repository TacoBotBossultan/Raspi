use std::fmt::Debug;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

pub trait ChassisTraits: Debug {
    fn set_motor_speeds(
        &mut self,
        front_right_motor_speed: EngineOrder,
        front_left_motor_speed: EngineOrder,
        back_left_motor_speed: EngineOrder,
        back_right_motor_speed: EngineOrder,
    );
    fn stop_motors(&mut self);
    fn get_position(&mut self) -> Result<Position, String>;
    fn insert_rack(&mut self);
    fn retrieve_rack(&mut self);
    fn are_buttons_pressed(&mut self) -> bool;
    fn arrived_at_a_lane(&mut self) -> bool;
    fn set_position(&mut self, position: Position);
    fn is_rack_inserted(&mut self) -> bool;
    fn is_rack_extracted(&mut self) -> bool;
    fn beer_me(&mut self);
    fn on_led(&mut self);
    fn off_led(&mut self);
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, FromPrimitive)]
pub enum MotorIndex {
    FrontRight = 0,
    FrontLeft = 1,
    BackLeft = 2,
    BackRight = 3,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, FromPrimitive, Serialize, Deserialize)]
pub enum EngineOrder {
    FullAhead = 200,
    SlowAhead = 105,
    UnDeadSlowAhead = 103,
    DeadSlowAhead = 102,
    Stop = 100,
    DeadSlowAstern = 98,
    SlowAstern = 95,
    FullAstern = 1,
}

impl Into<u8> for EngineOrder {
    fn into(self) -> u8 {
        self.clone() as u8
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub position_name: Option<String>,
    pub x_coordinate: i32,
    pub y_coordinate: i32,
    pub theta: u16,
}

impl Position {
    pub fn new(
        position_name: Option<String>,
        x_coordinate: i32,
        y_coordinate: i32,
        orientation: u16,
    ) -> Result<Position, String> {
        if orientation > 360 {
            return Err(
                "The robot's orientation may vary between 0 and 360 degrees, and that's it."
                    .to_string(),
            );
        }

        //let position_name = Some("o pozitie".to_string());

        Ok(Position {
            position_name,
            x_coordinate,
            y_coordinate,
            theta: orientation,
        })
    }

    pub fn get_position_name(&self) -> Option<String> {
        self.position_name.clone()
    }

    pub fn get_x_coordinate(&self) -> i32 {
        self.x_coordinate
    }

    pub fn get_y_coordinate(&self) -> i32 {
        self.y_coordinate
    }

    pub fn get_theta(&self) -> u16 {
        self.theta
    }

    pub fn set_position_name(&mut self, position_name: String) {
        self.position_name = Some(position_name);
    }

    pub fn equals(&self, other: &Position) -> bool {
        if self.x_coordinate.abs_diff(other.x_coordinate) <= 25
            && self.y_coordinate.abs_diff(other.y_coordinate) <= 25
            && self.theta.abs_diff(other.theta) <= 10
        {
            return true;
        }
        false
    }

    pub fn equals_coordinates(&self, other: &Position) -> bool {
        if self.x_coordinate.abs_diff(other.x_coordinate) <= 25
            && self.y_coordinate.abs_diff(other.y_coordinate) <= 25
        {
            return true;
        }
        false
    }

    pub fn equals_theta(&self, other: &Position) -> bool {
        self.theta.abs_diff(other.theta) <= 10
    }
}

impl From<(Option<&str>, i32, i32, u16)> for Position {
    fn from(item: (Option<&str>, i32, i32, u16)) -> Self {
        Position {
            position_name: item.0.map(|str| str.to_string()),
            x_coordinate: item.1,
            y_coordinate: item.2,
            theta: item.3,
        }
    }
}
