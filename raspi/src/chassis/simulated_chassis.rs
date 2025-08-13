use crossterm::terminal::Clear;
use crossterm::{cursor::MoveTo, execute};
use num_traits::FromPrimitive;

use super::chassis_traits::{ChassisTraits, EngineOrder, MotorIndex, Position};
use crate::navigation_computing::navigation_computer::LOOP_TIME;
use std::cmp::max;
use std::f32::consts::PI;
use std::{
    collections::HashMap,
    io::{Write, stdout},
};

static PRE_APPEND_STR: &str = "[Simulated-Chassis]";

#[derive(Debug)]
pub struct SimulatedChassis {
    motor_speeds: HashMap<MotorIndex, EngineOrder>,
    motor_efficiency_values: HashMap<MotorIndex, f32>,
    current_position: Position,
    chassis_data_positions: [(u16, u16); 4],
}

impl ChassisTraits for SimulatedChassis {
    fn set_motor_speeds(
        &mut self,
        front_right_motor_speed: EngineOrder,
        front_left_motor_speed: EngineOrder,
        back_left_motor_speed: EngineOrder,
        back_right_motor_speed: EngineOrder,
    ) {
        self.motor_speeds
            .insert(MotorIndex::FrontRight, front_right_motor_speed);
        self.motor_speeds
            .insert(MotorIndex::FrontLeft, front_left_motor_speed);
        self.motor_speeds
            .insert(MotorIndex::BackLeft, back_left_motor_speed);
        self.motor_speeds
            .insert(MotorIndex::BackRight, back_right_motor_speed);

        // self.simulate_position_change();

        execute!(stdout(), MoveTo(0, 4)).unwrap();
        print!("The current position is {:?}.", self.current_position);
        stdout().flush().unwrap();

        execute!(
            stdout(),
            MoveTo(
                self.chassis_data_positions[1].0,
                self.chassis_data_positions[1].1
            ),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();

        execute!(
            stdout(),
            MoveTo(
                self.chassis_data_positions[2].0,
                self.chassis_data_positions[2].1
            ),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();

        for (index, &(x, y)) in self.chassis_data_positions.iter().enumerate() {
            execute!(stdout(), MoveTo(x, y)).unwrap();

            if let Some(motor_index) = MotorIndex::from_u8(index as u8) {
                print!("{:?}", self.motor_speeds[&motor_index]);
            }
        }
    }

    fn get_position(&self) -> Position {
        let curpos = self.current_position.clone();

        execute!(
            stdout(),
            MoveTo(0, 23),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        print!("Pozitia curenta estee");

        execute!(
            stdout(),
            MoveTo(0, 24),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        print!("{curpos:?}");
        curpos
    }

    fn insert_rack(&self) {}

    fn retrieve_rack(&self) {}

    fn are_buttons_pressed(&self) -> bool {
        todo!()
    }

    fn arrived_at_a_lane(&self) -> bool {
        todo!()
    }

    fn set_position(&self, position: Position) {
        todo!()
    }

    fn is_rack_inserted(&self) -> bool {
        true
    }

    fn is_rack_extracted(&self) -> bool {
        true
    }

    fn stop_motors(&mut self) {}

    fn beer_me(&self) {
        todo!()
    }

    fn on_led(&self) {
        todo!()
    }

    fn off_led(&self) {
        todo!()
    }
}

impl SimulatedChassis {
    pub fn new() -> Self {
        let motor_speeds = HashMap::new();
        let motor_efficiency_values = HashMap::new();
        let current_position = Position::create(None, 0, 0, 0).unwrap();
        let chassis_data_positions = [
            (35, 5),  // top-right motor
            (15, 5),  // top-left motor
            (15, 13), // bottom-left motor
            (35, 13), // bottom-right motor
        ];
        SimulatedChassis {
            motor_speeds,
            motor_efficiency_values,
            current_position,
            chassis_data_positions,
        }
    }

    pub fn set_motor_efficiency(
        &mut self,
        motor_index: MotorIndex,
        efficiency: u16,
    ) -> Result<String, String> {
        if efficiency > 100 {
            return Err("We're not building a super-unity machine here.".to_string());
        }

        self.motor_efficiency_values
            .insert(motor_index.clone(), efficiency as f32 / 100.0);
        Ok(format!(
            "Motor {motor_index:?} efficiency successfully set to {efficiency}."
        ))
    }

    pub fn simulate_position_change(&mut self) {
        if self
            .motor_speeds
            .iter()
            .all(|kvp| *kvp.1 == EngineOrder::Stop)
        {
            return;
        }
        //o sa fie urat
        let front_right_speed = (self
            .motor_speeds
            .get(&MotorIndex::FrontRight)
            .unwrap()
            .clone() as i16
            - 100) as f32
            * *self
                .motor_efficiency_values
                .get(&MotorIndex::FrontRight)
                .unwrap();
        let front_left_speed = (self
            .motor_speeds
            .get(&MotorIndex::FrontLeft)
            .unwrap()
            .clone() as i16
            - 100) as f32
            * *self
                .motor_efficiency_values
                .get(&MotorIndex::FrontLeft)
                .unwrap();
        let back_left_speed = (self
            .motor_speeds
            .get(&MotorIndex::BackLeft)
            .unwrap()
            .clone() as i16
            - 100) as f32
            * *self
                .motor_efficiency_values
                .get(&MotorIndex::BackLeft)
                .unwrap();
        let back_right_speed = (self
            .motor_speeds
            .get(&MotorIndex::BackRight)
            .unwrap()
            .clone() as i16
            - 100) as f32
            * *self
                .motor_efficiency_values
                .get(&MotorIndex::BackRight)
                .unwrap();

        //false== inapoi , true == inainte
        let front_right_sense = front_right_speed >= 0.0;
        let back_left_sense = back_left_speed >= 0.0;
        let back_right_sense = back_right_speed >= 0.0;
        let front_left_sense = front_left_speed >= 0.0;

        let mut relative_x_movement: f32 = 0.0;
        let mut relative_y_movement: f32 = 0.0;
        let mut relative_theta_movement: f32 = 0.0;

        let a: f32;
        let motors_net_deviation: f32;
        let loop_seconds = LOOP_TIME as f32 / 1000.0;
        // experimentally found via
        // https://www.desmos.com/calculator/qli2x2pvun
        // https://www.desmos.com/calculator/uk6wbmemqd
        let scale_randem = 0.00005;
        let parabola = |x: f32, a: f32| (a * x * x);

        // linear / rotating (front / back / rotate left / rotate right)
        if front_right_sense == back_right_sense && front_left_sense == back_left_sense {
            let right_speed = front_right_speed + back_right_speed;
            let left_speed = front_left_speed + back_left_speed;

            if right_speed == -left_speed {
                relative_theta_movement = left_speed * PI / 60.0 * (LOOP_TIME as f32 / 1000.0);
            } else {
                motors_net_deviation = right_speed - left_speed;
                a = scale_randem * motors_net_deviation;
                let forward_speed_normal_units: f32 = right_speed + left_speed;

                relative_x_movement = forward_speed_normal_units * loop_seconds;
                relative_y_movement = parabola(relative_x_movement, a);
                relative_theta_movement -= (2.0 * a * relative_x_movement).atan();
            }
        }
        //strafe left / right
        else if front_right_sense == back_left_sense && front_left_sense == back_right_sense {
            let main_diagonal_speed = front_left_speed + back_right_speed;
            let secondary_diagonal_speed = front_right_speed + back_left_speed;

            motors_net_deviation = main_diagonal_speed.abs() - secondary_diagonal_speed.abs();
            a = scale_randem * motors_net_deviation;

            let leftward_net_movement = secondary_diagonal_speed - main_diagonal_speed;

            relative_y_movement = leftward_net_movement * loop_seconds;
            relative_x_movement = parabola(relative_y_movement, a);
            relative_theta_movement -= (2.0 * a * relative_x_movement).atan();
        } else {
            //nuj man ce exotic mi-ai dat aici
            print!("Woow thats too exotic of a movement idk where you'll be");
        }

        let curr_theta = (self.current_position.get_theta() as f32).to_radians();
        let rounded_sin = approx_angle(curr_theta.sin());
        let rounded_cos = approx_angle(curr_theta.cos());

        let absolute_x_change =
            relative_x_movement * rounded_cos + relative_y_movement * rounded_sin;
        let absolute_y_change =
            relative_y_movement * rounded_cos - relative_x_movement * rounded_sin;
        let absolute_theta_change = relative_theta_movement.to_degrees();

        execute!(
            stdout(),
            MoveTo(0, 25),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )
        .unwrap();
        print!(
            "absolute x change: {absolute_x_change:?}; absolute y change: {absolute_y_change:?}; absolute theta change: {absolute_theta_change:?}"
        );

        let approximated_absolute_x_movement = approx_movement(absolute_x_change);
        let approximated_absolute_y_movement = approx_movement(absolute_y_change);

        let new_x = (max(
            (self.current_position.get_x_coordinate() as f32 + approximated_absolute_x_movement)
                .ceil() as u32,
            0,
        )) as u32;
        let new_y = (max(
            (self.current_position.get_y_coordinate() as f32 + approximated_absolute_y_movement)
                .ceil() as u32,
            0,
        )) as u32;
        let new_theta = ((self.current_position.get_theta() as f32 + absolute_theta_change)
            .rem_euclid(360.0)) as u16;

        let new_pos: Position = Position::create(None, new_x, new_y, new_theta).unwrap();

        self.current_position = new_pos;
    }

    pub fn set_position(&mut self, position: Position) -> Result<String, String> {
        self.current_position = position.clone();
        Ok(format!("Robot's position successfully set to {position:?}"))
    }
}

fn approx_angle(n: f32) -> f32 {
    if n.abs() <= 0.0001 { 0.0 } else { n }
}

fn approx_movement(n: f32) -> f32 {
    if n == 0.0 {
        0.0
    } else {
        n / n.abs() * n.abs().ceil()
    }
}
