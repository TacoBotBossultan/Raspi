use std::{io::stdout, sync::Arc, time::Duration};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};
use tokio::{
    spawn,
    sync::{self, Notify},
    task::JoinHandle,
    time::interval,
};

use crate::{
    chassis::{
        chassis_traits::{ChassisTraits, Position},
        real_chassis::RealChassis,
    },
    image_recognition::electric_eye::LeRelevement,
    navigation_computing::nav_computer_states::NavComputerStates,
};

use super::nav_computer_states::{Direction, Stopped};

pub const LOOP_TIME: u64 = 100;
static PRE_APPEND_STR: &str = "[Navigation-Computer]";
static LANE_SEEK_STRING: Option<&'static str> = Some("LANE_SEEK");

#[derive(Debug)]
pub struct NavigationComputer {
    target_position: Arc<sync::Mutex<Position>>,
    current_position: Arc<sync::Mutex<Position>>,
    keep_going: Arc<sync::Mutex<bool>>,
    notify: Arc<Notify>,
}

impl NavigationComputer {
    pub fn new() -> Self {
        let init_position = Position::create(None, 0, 0, 0).unwrap();
        let current_position = Arc::new(sync::Mutex::new(init_position.clone()));
        let target_position = Arc::new(sync::Mutex::new(init_position));
        let keep_going = Arc::new(sync::Mutex::new(true));
        NavigationComputer {
            target_position,
            current_position,
            keep_going,
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn get_current_position(&self) -> Position {
        self.current_position.lock().await.clone()
    }

    pub async fn set_target_position(&self, new_position: Position) {
        *self.target_position.lock().await = new_position.clone();
    }

    pub fn start(self: Arc<Self>, chassis: Arc<sync::Mutex<RealChassis>>) -> JoinHandle<()> {
        let go = Arc::clone(&self.keep_going);
        spawn(async move {
            let mut ticker = interval(Duration::from_millis(LOOP_TIME));
            let mut keep_loop_going = true;
            let mut current_state = NavComputerStates::Stopped(Stopped::new());
            let mut chassis_lock = chassis.lock().await;
            (*chassis_lock).set_position(Position::create(None, 0, 0, 0).unwrap());
            drop(chassis_lock);
            while keep_loop_going {
                chassis_lock = chassis.lock().await;
                *self.current_position.lock().await = match (*chassis_lock).get_position() {
                    Ok(pos) => pos,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };

                drop(chassis_lock);
                let current_position = (*self.current_position.lock().await).clone();
                let target_position = (*self.target_position.lock().await).clone();

                if current_position.equals(&target_position) {
                    self.notify.notify_one();
                }

                execute!(
                    stdout(),
                    MoveTo(0, 1),
                    Clear(crossterm::terminal::ClearType::CurrentLine)
                )
                .unwrap();
                print!("{PRE_APPEND_STR} The target position is: {target_position:?}.");

                current_state = current_state
                    .run(target_position, current_position, Arc::clone(&chassis))
                    .await;

                ticker.tick().await;
                keep_loop_going = *go.lock().await;
            }
        })
    }

    pub async fn kill(&self) {
        *self.keep_going.lock().await = false;
    }

    pub async fn stop_moving(&self, chassis: Arc<sync::Mutex<RealChassis>>) {
        loop {
            let mut chassis_lock = chassis.lock().await;
            *self.current_position.lock().await = match (*chassis_lock).get_position() {
                Ok(pos) => pos,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
            break;
        }
        let curr_pos = self.current_position.lock().await;
        println!("pozitia curenta e : {curr_pos:#?}");
        *self.target_position.lock().await = curr_pos.clone();
    }

    pub async fn go_to_position(&self, new_position: Position) -> Result<String, String> {
        *self.target_position.lock().await = new_position.clone();
        println!("Target position successfully set to {new_position:?}.");
        self.notify.notified().await;
        Ok(format!(
            "Successfully arrived at position {new_position:?}."
        ))
    }

    pub async fn dock(&self, relevement: LeRelevement) {
        let distance = relevement.get_distance();
        let angle = 90.0 - (relevement.get_angle()).abs();
        let forward_backward_dist = (distance * angle.cos()) as i32;
        let strafe_dist = (distance * angle.sin()) as i32;

        let curr_position = (*self.current_position.lock().await).clone();
        let mut new_x = curr_position.get_x_coordinate();
        let mut new_y = curr_position.get_y_coordinate();

        let mut orientation = Direction::NoMovement;
        if relevement.get_angle() < 0.0 {
            orientation = Direction::Backward;
        } else if relevement.get_angle() > 0.0 {
            orientation = Direction::Forward;
        }

        match curr_position.get_theta() {
            0 => {
                new_y += strafe_dist;
                match orientation {
                    Direction::Forward => {
                        new_x += forward_backward_dist;
                    }
                    Direction::Backward => {
                        new_x -= forward_backward_dist;
                    }
                    _ => {}
                }
            }
            90 => {
                new_x += strafe_dist;
                match orientation {
                    Direction::Forward => {
                        new_y -= forward_backward_dist;
                    }
                    Direction::Backward => {
                        new_y += forward_backward_dist;
                    }
                    _ => {}
                }
            }
            180 => {
                new_y -= strafe_dist;
                match orientation {
                    Direction::Forward => {
                        new_x -= forward_backward_dist;
                    }
                    Direction::Backward => {
                        new_x += forward_backward_dist;
                    }
                    _ => {}
                }
            }
            270 => {
                new_x -= strafe_dist;
                match orientation {
                    Direction::Forward => {
                        new_y += forward_backward_dist;
                    }
                    Direction::Backward => {
                        new_y -= forward_backward_dist;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        let new_position = Position::create(None, new_x, new_y, curr_position.get_theta()).unwrap();
        *self.target_position.lock().await = new_position.clone();
    }

    pub async fn go_forward_slowly(&self, distance: i32) {
        let curr_pos = (*self.current_position.lock().await).clone();
        let mut new_x = curr_pos.get_x_coordinate();
        let mut new_y = curr_pos.get_y_coordinate();
        match curr_pos.get_theta() {
            0 => new_x += distance,
            90 => new_y -= distance,
            180 => new_x -= distance,
            270 => new_y += distance,
            _ => (),
        }
        let new_position = Position::create(
            LANE_SEEK_STRING.map(String::from),
            new_x,
            new_y,
            curr_pos.get_theta(),
        )
        .unwrap();
        *self.target_position.lock().await = new_position.clone();
    }

    pub async fn undock(&self, distance: i32) {
        let curr_pos = (*self.current_position.lock().await).clone();
        let mut new_x = curr_pos.get_x_coordinate();
        let mut new_y = curr_pos.get_y_coordinate();
        match curr_pos.get_theta() {
            0 => new_y -= distance,
            90 => new_x -= distance,
            180 => new_y += distance,
            270 => new_x += distance,
            _ => (),
        }
        let new_position = Position::create(None, new_x, new_y, curr_pos.get_theta()).unwrap();
        *self.target_position.lock().await = new_position.clone();
    }
}

impl Default for NavigationComputer {
    fn default() -> Self {
        Self::new()
    }
}
