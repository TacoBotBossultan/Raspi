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

#[derive(Debug, Deserialize, PartialEq)]
pub enum ActionType {
    GoToPosition,
    InsertRack,
    RemoveRack,
    TakePhoto,
}

#[derive(Debug)]
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
