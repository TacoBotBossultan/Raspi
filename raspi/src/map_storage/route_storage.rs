use serde::Deserialize;

use crate::chassis::chassis_traits::Position;
use crate::navigation_computing::nav_computer_states::{Direction, DirectionMove};
use crate::request_response::requests::StoreRouteRequest;
use std::collections::HashMap;
use std::fmt::Debug;

pub fn move_to_next_position(
    starting_position: Position,
    direction_move: DirectionMove,
) -> Result<Position, String> {
    let mut x_coord: i32 = starting_position.get_x_coordinate();
    let mut y_coord: i32 = starting_position.get_y_coordinate();
    let mut theta: u16 = starting_position.get_theta() as u16;

    match starting_position.get_theta() {
        0 => match direction_move.get_direction_type() {
            Direction::Forward => {
                x_coord = starting_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Backward => {
                x_coord = starting_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Left => {
                y_coord = starting_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Right => {
                y_coord = starting_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::RotateRight => {
                theta = (starting_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::RotateLeft => {
                theta = (starting_position.get_theta() + (360 - direction_move.get_value() as u16))
                    % 360
            }
            Direction::NoMovement => (),
        },

        90 => match direction_move.get_direction_type() {
            Direction::Forward => {
                y_coord = starting_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Backward => {
                y_coord = starting_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Left => {
                x_coord = starting_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Right => {
                x_coord = starting_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::RotateRight => {
                theta = (starting_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::RotateLeft => {
                theta = (starting_position.get_theta() + (360 - direction_move.get_value() as u16))
                    % 360
            }
            Direction::NoMovement => {}
        },

        180 => match direction_move.get_direction_type() {
            Direction::Forward => {
                x_coord = starting_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Backward => {
                x_coord = starting_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Left => {
                y_coord = starting_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Right => {
                y_coord = starting_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::RotateRight => {
                theta = (starting_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::RotateLeft => {
                theta = (starting_position.get_theta() + (360 - direction_move.get_value() as u16))
                    % 360
            }
            Direction::NoMovement => {}
        },

        270 => match direction_move.get_direction_type() {
            Direction::Forward => {
                y_coord = starting_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Backward => {
                y_coord = starting_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Left => {
                x_coord = starting_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Right => {
                x_coord = starting_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::RotateRight => {
                theta = (starting_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::RotateLeft => {
                theta = (starting_position.get_theta() + (360 - direction_move.get_value() as u16))
                    % 360
            }
            Direction::NoMovement => {}
        },

        _ => {}
    };
    Position::new(None, x_coord, y_coord, theta as u16)
}

pub fn move_to_previous_position(
    ending_position: Position,
    direction_move: DirectionMove,
) -> Result<Position, String> {
    let mut x_coord = ending_position.get_x_coordinate();
    let mut y_coord = ending_position.get_y_coordinate();
    let mut theta = ending_position.get_theta();

    match ending_position.get_theta() {
        0 => match direction_move.get_direction_type() {
            Direction::Forward => {
                x_coord = ending_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Backward => {
                x_coord = ending_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Left => {
                y_coord = ending_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Right => {
                y_coord = ending_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::RotateRight => {
                theta =
                    (ending_position.get_theta() + (360 - direction_move.get_value() as u16)) % 360
            }
            Direction::RotateLeft => {
                theta = (ending_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::NoMovement => {}
        },

        90 => match direction_move.get_direction_type() {
            Direction::Forward => {
                y_coord = ending_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Backward => {
                y_coord = ending_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Left => {
                x_coord = ending_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Right => {
                x_coord = ending_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::RotateRight => {
                theta =
                    (ending_position.get_theta() + (360 - direction_move.get_value() as u16)) % 360
            }
            Direction::RotateLeft => {
                theta = (ending_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::NoMovement => {}
        },

        180 => match direction_move.get_direction_type() {
            Direction::Forward => {
                x_coord = ending_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Backward => {
                x_coord = ending_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::Left => {
                y_coord = ending_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Right => {
                y_coord = ending_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::RotateRight => {
                theta =
                    (ending_position.get_theta() + (360 - direction_move.get_value() as u16)) % 360
            }
            Direction::RotateLeft => {
                theta = (ending_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::NoMovement => {}
        },

        270 => match direction_move.get_direction_type() {
            Direction::Forward => {
                y_coord = ending_position.get_y_coordinate() - direction_move.get_value()
            }
            Direction::Backward => {
                y_coord = ending_position.get_y_coordinate() + direction_move.get_value()
            }
            Direction::Left => {
                x_coord = ending_position.get_x_coordinate() + direction_move.get_value()
            }
            Direction::Right => {
                x_coord = ending_position.get_x_coordinate() - direction_move.get_value()
            }
            Direction::RotateRight => {
                theta =
                    (ending_position.get_theta() + (360 - direction_move.get_value() as u16)) % 360
            }
            Direction::RotateLeft => {
                theta = (ending_position.get_theta() + direction_move.get_value() as u16) % 360
            }
            Direction::NoMovement => {}
        },

        _ => {}
    };
    Position::new(None, x_coord, y_coord, theta as u16)
}

#[derive(Debug, Clone)]
pub struct PositionStorage {
    positions: Vec<Position>,
}

impl PositionStorage {
    pub fn new() -> Self {
        PositionStorage {
            positions: Vec::new(),
        }
    }

    pub fn store_position(&mut self, position: &Position) {
        self.positions.push(position.clone());
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.positions.clone()
    }

    pub fn search_by_name(&self, position_name: String) -> Result<Position, String> {
        let positions_clone = self.positions.clone();
        for position in positions_clone.into_iter() {
            if let Some(curr_position_name) = position.get_position_name()
                && curr_position_name == position_name
            {
                return Ok(position);
            }
        }
        Err("position not found.".to_string())
    }

    pub fn search_by_coordinates(
        &self,
        position_x: i32,
        position_y: i32,
        position_theta: u16,
    ) -> Result<Position, String> {
        let positions_clone = self.positions.clone();
        for position in positions_clone.into_iter() {
            if position.get_x_coordinate() == position_x
                && position.get_y_coordinate() == position_y
                && position.get_theta() == position_theta
            {
                return Ok(position);
            }
        }
        Err("position not found.".to_string())
    }

    pub fn is_in_storage_by_name(&self, position_name: String) -> bool {
        let positions_clone = self.positions.clone();
        for position in positions_clone.into_iter() {
            if let Some(curr_position_name) = position.get_position_name()
                && curr_position_name == position_name
            {
                return true;
            }
        }
        false
    }
}

impl Default for PositionStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RouteKey {
    start_name: String,
    destination_name: String,
}

impl RouteKey {
    pub fn new(start_name: String, destination_name: String) -> Self {
        RouteKey {
            start_name,
            destination_name,
        }
    }

    pub fn get_start_name(&self) -> String {
        self.start_name.clone()
    }

    pub fn get_destination_name(&self) -> String {
        self.destination_name.clone()
    }

    pub fn equals(&self, other: RouteKey) -> bool {
        self.start_name == other.start_name && self.destination_name == other.destination_name
    }

    pub fn equals_reversed(&self, other: RouteKey) -> bool {
        self.start_name == other.destination_name && self.destination_name == other.start_name
    }
}

#[derive(Debug, Clone)]
pub struct MapStorage {
    position_storage: PositionStorage,
    routes_by_end_points: HashMap<RouteKey, Vec<Position>>,
}

impl MapStorage {
    pub fn new() -> Self {
        MapStorage {
            position_storage: PositionStorage::new(),
            routes_by_end_points: HashMap::new(),
        }
    }

    pub fn store_position(&mut self, position: Position) {
        self.position_storage.store_position(&position);
    }

    pub fn compute_route_from_start(
        &mut self,
        start_position: Position,
        route_request: StoreRouteRequest,
    ) -> Result<Vec<Position>, String> {
        let mut positions_vector: Vec<Position> = Vec::new();
        let mut current_position = start_position.clone();
        positions_vector.push(start_position.clone());
        let mut route_request_vector = route_request.get_route().clone();

        while !route_request_vector.is_empty() {
            let current_direction_move = match route_request_vector.pop_front() {
                Some(dir_move) => dir_move,
                None => {
                    return Err("Invalid direction move".to_string());
                }
            };
            let mut next_position =
                match move_to_next_position(current_position.clone(), current_direction_move) {
                    Ok(next_pos) => next_pos,
                    Err(output) => {
                        return Err(output);
                    }
                };
            if route_request_vector.is_empty() {
                next_position.set_position_name(route_request.get_destination_position_name());
            }
            self.position_storage.store_position(&next_position);
            if next_position.get_position_name().is_some() {
                self.position_storage.store_position(&next_position);
            }
            positions_vector.push(next_position.clone());
            current_position = next_position;
        }
        Ok(positions_vector)
    }

    pub fn compute_route_from_end_request(
        &mut self,
        end_position: Position,
        route_request: StoreRouteRequest,
    ) -> Result<Vec<Position>, String> {
        let mut positions_vector: Vec<Position> = Vec::new();
        let mut current_position = end_position.clone();
        positions_vector.push(end_position.clone());
        let mut route_request_vector = route_request.get_route().clone();

        while !route_request_vector.is_empty() {
            let current_direction_move = match route_request_vector.pop_back() {
                Some(dir_move) => dir_move,
                None => {
                    return Err("Invalid direction move".to_string());
                }
            };
            let mut prev_position =
                match move_to_previous_position(current_position.clone(), current_direction_move) {
                    Ok(prev_pos) => prev_pos,
                    Err(output) => {
                        return Err(output);
                    }
                };
            if route_request_vector.is_empty() {
                prev_position.set_position_name(route_request.get_start_position_name());
            }
            if prev_position.get_position_name().is_some() {
                self.position_storage.store_position(&prev_position);
            }
            positions_vector.insert(0, prev_position.clone());
            current_position = prev_position;
        }
        Ok(positions_vector)
    }

    pub fn store_route(&mut self, route_request: &StoreRouteRequest) -> Result<String, String> {
        if route_request.get_route().is_empty() {
            return Err("Invalid route.".to_string());
        }

        let new_route_key: RouteKey = RouteKey::new(
            route_request.get_start_position_name(),
            route_request.get_destination_position_name(),
        );

        let start_position: Position;
        let end_position: Position;
        let mut positions_vector: Vec<Position> = Vec::new();
        if self
            .position_storage
            .is_in_storage_by_name(new_route_key.start_name.clone())
        {
            start_position = self
                .position_storage
                .search_by_name(new_route_key.start_name.clone())?;
            positions_vector = self
                .compute_route_from_start_request(start_position.clone(), route_request.clone())?;
        } else if self
            .position_storage
            .is_in_storage_by_name(new_route_key.destination_name.clone())
        {
            end_position = self
                .position_storage
                .search_by_name(new_route_key.destination_name.clone())?;
            positions_vector =
                self.compute_route_from_end_request(end_position.clone(), route_request.clone())?;
        } else {
            return Err("Start and end positions unknown.".to_string());
        }
        self.routes_by_end_points
            .insert(new_route_key, positions_vector);

        Ok("The route was stored successfully".to_string())
    }

    pub fn get_route(&self, route_key: &RouteKey) -> Result<Vec<Position>, String> {
        match self.routes_by_end_points.get(&route_key) {
            Some(value) => Ok(value.clone()),
            None => Err("no route found".to_string()),
        }
    }
}

impl Default for MapStorage {
    fn default() -> Self {
        Self::new()
    }
}
