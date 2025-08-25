use crate::{chassis::chassis_traits::Position, map_storage::route_storage::RouteKey};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoToPosition {
    pub route: RouteKey,
}

#[derive(Debug, Deserialize)]
pub struct InsertRack {
    pub route: RouteKey,
}

#[derive(Debug, Deserialize)]
pub struct RemoveRack {
    pub route: RouteKey,
}

#[derive(Debug, Deserialize)]
pub struct TakePhoto {
    pub route: RouteKey,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ActionType {
    GoToPosition,
    InsertRack { lane_number: u8 },
    RemoveRack { lane_number: u8 },
    TakePhoto,
    BeerMe,
}

#[derive(Debug, Clone)]
pub struct ExecutableMission {
    pub action: ActionType,
    pub route: Vec<Position>,
}

#[derive(Debug)]
pub enum MissionStatus {
    Completed,
    NotCompleted,
}

impl ExecutableMission {
    pub fn new(action: ActionType, route: Vec<Position>) -> ExecutableMission {
        ExecutableMission { action, route }
    }
}
