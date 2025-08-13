use std::collections::VecDeque;

use super::super::{
    chassis::chassis_traits::Position,
    map_storage::route_storage::{MapStorage, RouteKey},
    navigation_computing::nav_computer_states::{Direction, DirectionMove},
    request_response::requests::StoreRoute,
};

pub struct StoreRouteHelper {
    pub map_storage: MapStorage,
}

impl StoreRouteHelper {
    pub fn new() -> Self {
        Self {
            map_storage: MapStorage::new(),
        }
    }

    pub fn store_route<const N: usize>(
        &mut self,
        route: ([(Direction, u32); N], &str, &str),
    ) -> Result<String, String> {
        let directions_arr = route.0;
        let start_name = route.1.to_string();
        let destination_name = route.2.to_string();

        let route_vecdeque = Self::route_arr_to_vecdeque(directions_arr);

        let route_request = StoreRoute::new(start_name, route_vecdeque, destination_name);

        self.map_storage.store_route(&route_request)
    }

    pub fn get_route(&mut self, route_key: (&str, &str)) -> Result<Vec<Position>, String> {
        let start_name = route_key.0.to_string();
        let destination_name = route_key.1.to_string();

        let route_key = RouteKey::new(start_name.to_string(), destination_name.to_string());
        self.map_storage.get_route(&route_key)
    }

    // il folosesti si la home gen
    pub fn store_position(&mut self, position: (Option<&str>, u32, u32, u16)) {
        self.map_storage.store_position(Position::from(position));
    }

    pub fn define_home(&mut self, position: (u32, u32, u16)) {
        self.store_position((Some("Home"), position.0, position.1, position.2));
    }

    pub fn route_arr_to_vecdeque<const N: usize>(
        directions_arr: [(Direction, u32); N],
    ) -> VecDeque<DirectionMove> {
        directions_arr
            .into_iter()
            .map(DirectionMove::from)
            .collect()
    }

    pub fn arr_to_position_vector<const N: usize>(
        dir_arr: [(Option<&str>, u32, u32, u16); N],
    ) -> Vec<Position> {
        dir_arr.into_iter().map(Position::from).collect()
    }
}

impl Default for StoreRouteHelper {
    fn default() -> Self {
        Self::new()
    }
}
