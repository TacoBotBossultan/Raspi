use crate::{
    map_storage::route_storage::RouteKey, mission_controller::missions::ActionType,
    navigation_computing::nav_computer_states::DirectionMove,
};
use serde::Deserialize;
use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug, Deserialize, PartialEq)]
pub struct PhotoRequest;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StateReqest;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct StoreRouteRequest {
    start_position_name: String,
    route: VecDeque<DirectionMove>,
    destination_position_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MissionRequest {
    pub action: ActionType,
    pub route: RouteType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DefineHomeRequest {
    home_x: i32,
    home_y: i32,
    home_theta: u16,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
pub enum RouteType {
    RouteKey(RouteKey),
    AbsolutePosition {
        x_coordinate: i32,
        y_coordinate: i32,
        theta: u16,
    },
    RelativeMovement(VecDeque<DirectionMove>),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
pub enum Requests {
    PhotoReqest(PhotoRequest),
    StateReqest(StateReqest),
    StoreRouteRequest(StoreRouteRequest),
    MissionRequest(MissionRequest),
    DefineHomeRequest(DefineHomeRequest),
}
impl PhotoRequest {
    pub fn new() -> PhotoRequest {
        print!("Photo Request!");
        PhotoRequest
    }
}

impl Default for PhotoRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl StateReqest {
    pub fn new() -> StateReqest {
        print!("State Request!");
        StateReqest
    }
}

impl Default for StateReqest {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreRouteRequest {
    pub fn new(
        start_position_name: String,
        route: VecDeque<DirectionMove>,
        destination_position_name: String,
    ) -> StoreRouteRequest {
        print!("Store Route Request!");
        StoreRouteRequest {
            start_position_name,
            route,
            destination_position_name,
        }
    }

    pub fn get_start_position_name(&self) -> String {
        self.start_position_name.clone()
    }
    pub fn get_route(&self) -> VecDeque<DirectionMove> {
        self.route.clone()
    }
    pub fn get_destination_position_name(&self) -> String {
        self.destination_position_name.clone()
    }
}

impl DefineHomeRequest {
    pub fn new(home_x: i32, home_y: i32, home_theta: u16) -> DefineHomeRequest {
        print!("Define Home Request!");
        DefineHomeRequest {
            home_x,
            home_y,
            home_theta,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.home_x
    }
    pub fn get_y(&self) -> i32 {
        self.home_y
    }
    pub fn get_theta(&self) -> u16 {
        self.home_theta
    }
}

impl MissionRequest {
    pub fn new(action: ActionType, start_name: String, destination_name: String) -> Self {
        MissionRequest {
            action,
            route: RouteType::RouteKey(RouteKey::new(start_name, destination_name)),
        }
    }
}
