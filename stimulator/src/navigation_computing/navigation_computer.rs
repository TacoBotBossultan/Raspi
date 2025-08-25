use std::{io::stdout, sync::Arc, time::Duration};

use crossterm::{cursor::MoveTo, execute, terminal::Clear};
use tokio::{spawn, sync, task::JoinHandle, time::interval};

use crate::{
    chassis::{
        chassis_traits::{ChassisTraits, Position},
        simulated_chassis::SimulatedChassis,
    },
    navigation_computing::nav_computer_states::NavComputerStates,
};

use super::nav_computer_states::Stopped;

pub const LOOP_TIME: u64 = 40;
static PRE_APPEND_STR: &str = "[Navigation-Computer]";

#[derive(Debug)]
pub struct NavigationComputer {
    target_position: Arc<sync::Mutex<Position>>,
    current_position: Arc<sync::Mutex<Position>>,
    keep_going: Arc<sync::Mutex<bool>>,
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
        }
    }

    pub fn start(self: Arc<Self>, chassis: Arc<sync::Mutex<SimulatedChassis>>) -> JoinHandle<()> {
        let go = Arc::clone(&self.keep_going);
        spawn(async move {
            let mut ticker = interval(Duration::from_millis(LOOP_TIME));
            let mut keep_loop_going = true;
            let mut current_state = NavComputerStates::Stopped(Stopped::new());
            while keep_loop_going {
                (*chassis.lock().await).simulate_position_change();

                *self.current_position.lock().await = (*chassis.lock().await).get_position();
                let current_position = (*self.current_position.lock().await).clone();
                let target_position = (*self.target_position.lock().await).clone();
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

    pub async fn stop(&self) {
        *self.keep_going.lock().await = false;
    }

    pub async fn go_to_position(&self, new_position: Position) -> Result<String, String> {
        *self.target_position.lock().await = new_position.clone();
        Ok(format!(
            "Target position successfully set to {new_position:?}."
        ))
    }
}

impl Default for NavigationComputer {
    fn default() -> Self {
        Self::new()
    }
}
