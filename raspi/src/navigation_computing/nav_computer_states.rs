use crossterm::{cursor::MoveTo, execute, terminal::Clear};
use serde::Deserialize;
use std::{fmt::Debug, io::stdout, sync::Arc};
use tokio::sync;

use crate::chassis::{
    chassis_traits::{ChassisTraits, EngineOrder, Position},
    real_chassis::RealChassis,
};

static LANE_SEEK_STRING: Option<&'static str> = Some("LANE_SEEK");

#[derive(Debug)]
pub struct Stopped;
#[derive(Debug)]
pub struct Accelerating {
    movement_forward_backward: Direction,
    movement_right_left: Direction,
    movement_on_theta: Direction,
    is_strafe_on_x: bool,
    decelerating_position: Position,
    initial_position: Position,
}

#[derive(Debug)]
pub struct Decelerating {
    movement_forward_backward: Direction,
    movement_right_left: Direction,
    movement_on_theta: Direction,
    is_strafe_on_x: bool,
    previous_position: Position,
    initial_position: Position,
}

#[derive(Debug)]
pub struct SlowRide {
    movement_forward_backward: Direction,
    movement_right_left: Direction,
    movement_on_theta: Direction,
    is_strafe_on_x: bool,
    initial_position: Position,
}

#[derive(Debug)]
pub struct Strafing {
    movement_right_left: Direction,
    movement_on_theta: Direction,
    initial_position: Position,
    is_strafe_on_x: bool,
}

#[derive(Debug)]
pub struct Rotating {
    movement_on_theta: Direction,
}

#[derive(Debug)]
pub enum NavComputerStates {
    Stopped(Stopped),
    Accelerating(Accelerating),
    Decelerating(Decelerating),
    SlowRide(SlowRide),
    Strafing(Strafing),
    Rotating(Rotating),
}

static PRE_APPEND_STR: &str = "[NavComputerStates]";

pub async fn stop_motors(chassis: &Arc<sync::Mutex<RealChassis>>) {
    chassis.lock().await.set_motor_speeds(
        EngineOrder::Stop,
        EngineOrder::Stop,
        EngineOrder::Stop,
        EngineOrder::Stop,
    );
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Direction {
    NoMovement,
    Forward,
    Right,
    Backward,
    Left,
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DirectionMove {
    direction_type: Direction,
    value: i32,
}

impl DirectionMove {
    pub fn new(direction_type: Direction, value: i32) -> Self {
        DirectionMove {
            direction_type,
            value,
        }
    }

    pub fn get_direction_type(&self) -> Direction {
        self.direction_type.clone()
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl From<(Direction, i32)> for DirectionMove {
    fn from(item: (Direction, i32)) -> Self {
        DirectionMove {
            direction_type: item.0,
            value: item.1,
        }
    }
}

impl Direction {
    fn from_value(value: i16) -> Direction {
        match value {
            0 => Direction::Forward,
            1 => Direction::Right,
            2 => Direction::Backward,
            3 => Direction::Left,
            _ => Direction::NoMovement,
        }
    }

    fn to_value(direction: Direction) -> i16 {
        match direction {
            Direction::Forward => 0,
            Direction::Right => 1,
            Direction::Backward => 2,
            Direction::Left => 3,
            Direction::RotateLeft => 4,
            Direction::RotateRight => 5,
            Direction::NoMovement => -1,
        }
    }

    async fn motors_setting(
        &self,
        is_slow: bool,
        is_leaning_into_instrument: bool,
        chassis: &Arc<sync::Mutex<RealChassis>>,
    ) {
        match self {
            Direction::Forward if !is_slow && !is_leaning_into_instrument => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::FullAhead,
                    EngineOrder::FullAhead,
                    EngineOrder::FullAhead,
                    EngineOrder::FullAhead,
                );
            }
            Direction::Forward if is_leaning_into_instrument => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::UnDeadSlowAhead,
                    EngineOrder::DeadSlowAhead,
                    EngineOrder::DeadSlowAhead,
                    EngineOrder::UnDeadSlowAhead,
                );
            }
            Direction::Forward => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAhead,
                );
            }
            Direction::Backward if !is_slow => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::FullAstern,
                    EngineOrder::FullAstern,
                    EngineOrder::FullAstern,
                    EngineOrder::FullAstern,
                );
            }
            Direction::Backward => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAstern,
                );
            }
            Direction::Left => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAhead,
                );
            }
            Direction::Right => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAstern,
                );
            }
            Direction::RotateRight => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAstern,
                );
            }
            Direction::RotateLeft => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::SlowAhead,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAstern,
                    EngineOrder::SlowAhead,
                );
            }
            Direction::NoMovement => {
                chassis.lock().await.set_motor_speeds(
                    EngineOrder::Stop,
                    EngineOrder::Stop,
                    EngineOrder::Stop,
                    EngineOrder::Stop,
                );
            }
        };
    }
}

pub enum Heading {
    North,
    East,
    South,
    West,
    NoClearOrientation,
}

impl Heading {
    fn from_degrees(deg: u16) -> Heading {
        match deg % 360 {
            0 => Heading::North,
            90 => Heading::East,
            180 => Heading::South,
            270 => Heading::West,
            _ => Heading::NoClearOrientation,
        }
    }
}
impl NavComputerStates {
    pub async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        match self {
            NavComputerStates::Stopped(state) => {
                state.run(target_position, current_position, chassis).await
            }
            NavComputerStates::Accelerating(state) => {
                state.run(target_position, current_position, chassis).await
            }
            NavComputerStates::Decelerating(state) => {
                state.run(target_position, current_position, chassis).await
            }
            NavComputerStates::SlowRide(state) => {
                state.run(target_position, current_position, chassis).await
            }
            NavComputerStates::Strafing(state) => {
                state.run(target_position, current_position, chassis).await
            }
            NavComputerStates::Rotating(state) => {
                state.run(target_position, current_position, chassis).await
            }
        }
    }
}

fn print_state(state: &str) {
    execute!(
        stdout(),
        MoveTo(33, 2),
        Clear(crossterm::terminal::ClearType::UntilNewLine)
    )
    .unwrap();
    execute!(stdout(), MoveTo(0, 2)).unwrap();
    print!("{PRE_APPEND_STR} The current nav computer state is {state}.");
}

impl Stopped {
    pub fn new() -> Stopped {
        print_state("Stopped");
        Stopped
    }
}

impl Default for Stopped {
    fn default() -> Self {
        Self::new()
    }
}

impl Accelerating {
    fn new(
        movement_forward_backward: Direction,
        movement_right_left: Direction,
        movement_on_theta: Direction,
        is_strafe_on_x: bool,
        decelerating_position: Position,
        initial_position: Position,
    ) -> Self {
        print_state("Accelerating");
        Accelerating {
            movement_forward_backward,
            movement_right_left,
            movement_on_theta,
            is_strafe_on_x,
            decelerating_position,
            initial_position,
        }
    }
}

impl Decelerating {
    fn new(
        movement_forward_backward: Direction,
        movement_right_left: Direction,
        movement_on_theta: Direction,
        is_strafe_on_x: bool,
        previous_position: Position,
        initial_position: Position,
    ) -> Decelerating {
        print_state("Decelerating");
        Decelerating {
            movement_forward_backward,
            movement_right_left,
            movement_on_theta,
            is_strafe_on_x,
            previous_position,
            initial_position,
        }
    }
}

impl SlowRide {
    fn new(
        movement_forward_backward: Direction,
        movement_right_left: Direction,
        movement_on_theta: Direction,
        is_strafe_on_x: bool,
        initial_position: Position,
    ) -> SlowRide {
        print_state("SlowRide");
        SlowRide {
            movement_forward_backward,
            movement_right_left,
            movement_on_theta,
            is_strafe_on_x,
            initial_position,
        }
    }
}

impl Strafing {
    fn new(
        movement_right_left: Direction,
        movement_on_theta: Direction,
        initial_position: Position,
        is_strafe_on_x: bool,
    ) -> Strafing {
        print_state("Strafing");
        Strafing {
            movement_right_left,
            movement_on_theta,
            initial_position,
            is_strafe_on_x,
        }
    }
}

impl Rotating {
    fn new(movement_on_theta: Direction) -> Rotating {
        print_state("Rotating");
        Rotating { movement_on_theta }
    }
}

impl RunnableNavState for Stopped {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        stop_motors(&chassis).await;
        let difference_x: i32 =
            target_position.get_x_coordinate() - current_position.get_x_coordinate();
        let difference_y: i32 =
            target_position.get_y_coordinate() - current_position.get_y_coordinate();
        let difference_theta =
            target_position.get_theta() as i32 - current_position.get_theta() as i32;
        let decelerating_position_x: i32 =
            8 * (difference_x) / 10 + current_position.get_x_coordinate();
        let decelerating_position_y: i32 =
            8 * (difference_y) / 10 + current_position.get_y_coordinate();

        let decelerating_position = Position::create(
            None,
            decelerating_position_x,
            decelerating_position_y,
            current_position.get_theta(),
        )
        .unwrap();

        let mut movement_on_x: Direction;
        let mut movement_on_y: Direction;

        if difference_x > 0 {
            movement_on_x = Direction::Forward;
        } else {
            movement_on_x = Direction::Backward;
        }

        if difference_y > 0 {
            movement_on_y = Direction::Right;
        } else {
            movement_on_y = Direction::Left;
        }

        let movement_on_theta = if difference_theta < 0 {
            Direction::RotateLeft
        } else {
            Direction::RotateRight
        };

        match Heading::from_degrees(current_position.get_theta()) {
            Heading::North => {}
            Heading::East => {
                movement_on_x = Direction::from_value((Direction::to_value(movement_on_x) + 1) % 4);
                movement_on_y = Direction::from_value((Direction::to_value(movement_on_y) + 1) % 4);
            }
            Heading::South => {
                movement_on_x = Direction::from_value((Direction::to_value(movement_on_x) + 2) % 4);
                movement_on_y = Direction::from_value((Direction::to_value(movement_on_y) + 2) % 4);
            }
            Heading::West => {
                movement_on_x = Direction::from_value((Direction::to_value(movement_on_x) + 3) % 4);
                movement_on_y = Direction::from_value((Direction::to_value(movement_on_y) + 3) % 4);
            }
            Heading::NoClearOrientation => {
                movement_on_x = Direction::NoMovement;
                movement_on_y = Direction::NoMovement;
            }
        }

        let mut movement_forward_backward = Direction::NoMovement;
        let mut movement_right_left = Direction::NoMovement;
        let mut is_strafe_on_x: bool = false;

        if movement_on_y == Direction::Forward || movement_on_y == Direction::Backward {
            movement_forward_backward = movement_on_y;
            movement_right_left = movement_on_x;
            is_strafe_on_x = true;
            if difference_y.abs() <= 100 {
                return NavComputerStates::SlowRide(SlowRide::new(
                    movement_forward_backward,
                    movement_right_left,
                    movement_on_theta,
                    is_strafe_on_x,
                    current_position,
                ));
            }
        } else if movement_on_x == Direction::Forward || movement_on_x == Direction::Backward {
            movement_forward_backward = movement_on_x;
            movement_right_left = movement_on_y;
            if difference_x.abs() <= 100 {
                return NavComputerStates::SlowRide(SlowRide::new(
                    movement_forward_backward,
                    movement_right_left,
                    movement_on_theta,
                    is_strafe_on_x,
                    current_position,
                ));
            }
        } else {
            return NavComputerStates::SlowRide(SlowRide::new(
                movement_forward_backward,
                movement_right_left,
                movement_on_theta,
                is_strafe_on_x,
                current_position,
            ));
        }
        NavComputerStates::Accelerating(Accelerating::new(
            movement_forward_backward,
            movement_right_left,
            movement_on_theta,
            is_strafe_on_x,
            decelerating_position,
            current_position,
        ))
    }
}

impl RunnableNavState for Accelerating {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        if Direction::NoMovement == self.movement_forward_backward {
            return NavComputerStates::Decelerating(Decelerating::new(
                self.movement_forward_backward,
                self.movement_right_left,
                self.movement_on_theta,
                self.is_strafe_on_x,
                current_position,
                self.initial_position,
            ));
        }
        self.movement_forward_backward
            .motors_setting(false, false, &chassis)
            .await;

        let mut difference_target_current: u32 = target_position
            .get_x_coordinate()
            .abs_diff(current_position.get_x_coordinate());

        let mut difference_target_deceleration: u32 = target_position
            .get_x_coordinate()
            .abs_diff(self.decelerating_position.get_x_coordinate());

        if self.is_strafe_on_x {
            difference_target_current = target_position
                .get_y_coordinate()
                .abs_diff(current_position.get_y_coordinate());
            difference_target_deceleration = target_position
                .get_y_coordinate()
                .abs_diff(self.decelerating_position.get_y_coordinate());
        }

        if difference_target_deceleration >= difference_target_current {
            return NavComputerStates::Decelerating(Decelerating::new(
                self.movement_forward_backward,
                self.movement_right_left,
                self.movement_on_theta,
                self.is_strafe_on_x,
                current_position,
                self.initial_position,
            ));
        }

        NavComputerStates::Accelerating(Accelerating::new(
            self.movement_forward_backward,
            self.movement_right_left,
            self.movement_on_theta,
            self.is_strafe_on_x,
            self.decelerating_position,
            self.initial_position,
        ))
    }
}

impl RunnableNavState for Decelerating {
    async fn run(
        self,
        _target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        if Direction::NoMovement == self.movement_forward_backward {
            return NavComputerStates::SlowRide(SlowRide::new(
                self.movement_forward_backward,
                self.movement_right_left,
                self.movement_on_theta,
                self.is_strafe_on_x,
                self.initial_position,
            ));
        }

        stop_motors(&chassis).await;

        if current_position.equals_coordinates(&self.previous_position) {
            return NavComputerStates::SlowRide(SlowRide::new(
                self.movement_forward_backward,
                self.movement_right_left,
                self.movement_on_theta,
                self.is_strafe_on_x,
                self.initial_position,
            ));
        }

        NavComputerStates::Decelerating(Decelerating::new(
            self.movement_forward_backward,
            self.movement_right_left,
            self.movement_on_theta,
            self.is_strafe_on_x,
            current_position,
            self.initial_position,
        ))
    }
}

impl RunnableNavState for SlowRide {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        if Direction::NoMovement == self.movement_forward_backward {
            return NavComputerStates::Strafing(Strafing::new(
                self.movement_right_left,
                self.movement_on_theta,
                self.initial_position,
                self.is_strafe_on_x,
            ));
        }
        self.movement_forward_backward
            .motors_setting(
                true,
                target_position.position_name.as_deref() == LANE_SEEK_STRING,
                &chassis,
            )
            .await;

        let mut difference_current_initial: u32 = self
            .initial_position
            .get_x_coordinate()
            .abs_diff(current_position.get_x_coordinate());

        let mut difference_target_initial: u32 = target_position
            .get_x_coordinate()
            .abs_diff(self.initial_position.get_x_coordinate());

        let mut difference_current_target: u32 = current_position
            .get_x_coordinate()
            .abs_diff(target_position.get_x_coordinate());

        if self.is_strafe_on_x {
            difference_target_initial = self
                .initial_position
                .get_y_coordinate()
                .abs_diff(target_position.get_y_coordinate());
            difference_current_initial = current_position
                .get_y_coordinate()
                .abs_diff(self.initial_position.get_y_coordinate());
            difference_current_target = current_position
                .get_y_coordinate()
                .abs_diff(target_position.get_y_coordinate());
        }

        if difference_current_target <= 1 || difference_current_initial > difference_target_initial
        {
            stop_motors(&chassis).await;
            return NavComputerStates::Strafing(Strafing::new(
                self.movement_right_left,
                self.movement_on_theta,
                self.initial_position,
                self.is_strafe_on_x,
            ));
        }
        NavComputerStates::SlowRide(SlowRide::new(
            self.movement_forward_backward,
            self.movement_right_left,
            self.movement_on_theta,
            self.is_strafe_on_x,
            self.initial_position,
        ))
    }
}

impl RunnableNavState for Strafing {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        if Direction::NoMovement == self.movement_right_left {
            return NavComputerStates::Rotating(Rotating::new(self.movement_on_theta));
        }

        self.movement_right_left
            .motors_setting(true, false, &chassis)
            .await;

        let mut difference_current_initial: u32 = self
            .initial_position
            .get_y_coordinate()
            .abs_diff(current_position.get_y_coordinate());

        let mut difference_target_initial: u32 = target_position
            .get_y_coordinate()
            .abs_diff(self.initial_position.get_y_coordinate());

        if self.is_strafe_on_x {
            difference_target_initial = self
                .initial_position
                .get_x_coordinate()
                .abs_diff(target_position.get_x_coordinate());
            difference_current_initial = current_position
                .get_x_coordinate()
                .abs_diff(self.initial_position.get_x_coordinate());
        }

        if current_position.equals_coordinates(&target_position)
            || difference_current_initial > difference_target_initial
        {
            stop_motors(&chassis).await;
            return NavComputerStates::Rotating(Rotating::new(self.movement_on_theta));
        }

        NavComputerStates::Strafing(Strafing::new(
            self.movement_right_left,
            self.movement_on_theta,
            self.initial_position,
            self.is_strafe_on_x,
        ))
    }
}
impl RunnableNavState for Rotating {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates {
        if current_position.equals_theta(&target_position) {
            stop_motors(&chassis).await;
            return NavComputerStates::Stopped(Stopped::new());
        }

        self.movement_on_theta
            .motors_setting(false, false, &chassis)
            .await;

        NavComputerStates::Rotating(Rotating::new(self.movement_on_theta))
    }
}

pub trait RunnableNavState: Debug + Send + Sync {
    async fn run(
        self,
        target_position: Position,
        current_position: Position,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> NavComputerStates;
}
