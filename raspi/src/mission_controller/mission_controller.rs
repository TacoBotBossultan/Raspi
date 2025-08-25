use std::{sync::Arc, time::Duration};

use crate::{
    chassis::{chassis_traits::ChassisTraits, real_chassis::RealChassis}, // serial_communication::SerialCommunicator
    image_recognition::electric_eye::ElectricEye,
    mission_controller::missions::{ActionType, ExecutableMission, MissionStatus},
    navigation_computing::navigation_computer::NavigationComputer,
    utils::logging::AsyncLogger,
};
use tokio::{
    spawn,
    sync::{self, MutexGuard, mpsc},
    task::JoinHandle,
    time::sleep,
};

pub struct MissionController {
    async_logger: AsyncLogger,
    keep_going: Arc<sync::Mutex<bool>>,
}

static PRE_APPEND_STR: &str = "[Mission-Controller]";
static SHORT_SLEEP: Duration = Duration::from_millis(100);
static LONG_SLEEP: Duration = Duration::from_millis(500);

impl MissionController {
    pub async fn stop(&self) {
        *self.keep_going.lock().await = false;
    }

    pub fn new(async_logger: AsyncLogger) -> Self {
        let keep_going = Arc::new(sync::Mutex::new(true));
        MissionController {
            async_logger,
            keep_going,
        }
    }

    pub async fn run(
        self: Arc<Self>,
        mut mission_receiver: mpsc::Receiver<ExecutableMission>,
        status_sender: mpsc::Sender<MissionStatus>,
        navigation_computer: Arc<NavigationComputer>,
        chassis: Arc<sync::Mutex<RealChassis>>,
    ) -> JoinHandle<()> {
        self.async_logger
            .out_print(format!("{PRE_APPEND_STR} Running.."))
            .await;
        let go = Arc::clone(&self.keep_going);
        spawn(async move {
            let nav_computer_handle = navigation_computer.clone().start(chassis.clone());
            let mut keep_loop_going = true;
            let chassis_clone = chassis.clone();
            while keep_loop_going {
                let mission = mission_receiver.recv().await.unwrap();
                self.async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR} Received new mission: {mission:#?}"
                    ))
                    .await;

                self.async_logger
                    .out_print(format!("{PRE_APPEND_STR} Starting mission execution..."))
                    .await;

                self.async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR}  set PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1Merg pe ruta:"
                    ))
                    .await;

                self.async_logger
                    .out_print(format!("{0:#?}", mission.route))
                    .await;

                for position in mission.route {
                    let _ = navigation_computer.go_to_position(position).await;
                }

                match mission.action {
                    ActionType::GoToPosition => {
                        self.async_logger
                            .out_print(format!("{PRE_APPEND_STR} si atat lol"))
                            .await;
                        continue;
                    }
                    ActionType::TakePhoto => {
                        let mut chassis_lock = chassis.lock().await;
                        chassis_lock.on_led();
                        drop(chassis_lock);
                        let _photo = ElectricEye::take_photo();
                        let mut chassis_lock = chassis.lock().await;
                        chassis_lock.off_led();
                        drop(chassis_lock);
                        self.async_logger
                            .out_print(format!("{PRE_APPEND_STR} si fac si-o poza"))
                            .await;
                        continue;
                    }
                    ActionType::InsertRack { lane_number } => {
                        if !self.dock(&navigation_computer, &chassis).await {
                            continue;
                        }
                        let mut chassis_lock: MutexGuard<'_, RealChassis>;
                        let mut lane_nr = 0;
                        let mut arrived_at_desired_lane = false;
                        while !arrived_at_desired_lane {
                            navigation_computer.go_forward_slowly(100).await;
                            let mut arrived_at_a_lane = false;
                            while !arrived_at_a_lane {
                                chassis_lock = chassis.lock().await;
                                arrived_at_a_lane = chassis_lock.arrived_at_a_lane();
                                drop(chassis_lock);
                                sleep(SHORT_SLEEP).await;
                            }
                            navigation_computer.stop_moving(chassis.clone()).await;
                            chassis_lock = chassis.lock().await;
                            chassis_lock.stop_motors();
                            drop(chassis_lock);
                            lane_nr += 1;
                            if lane_nr == lane_number {
                                chassis_lock = chassis.lock().await;
                                chassis_lock.insert_rack();
                                drop(chassis_lock);
                                let mut is_rack_inserted = false;
                                while !is_rack_inserted {
                                    chassis_lock = chassis.lock().await;
                                    is_rack_inserted = chassis_lock.is_rack_inserted();
                                    drop(chassis_lock);
                                    sleep(LONG_SLEEP).await;
                                }
                                arrived_at_desired_lane = true;
                                navigation_computer.undock(500).await;
                                chassis_lock = chassis.lock().await;
                                chassis_lock.retrieve_rack();
                                drop(chassis_lock);
                                let mut is_dt_inserted = false;
                                while !is_dt_inserted {
                                    chassis_lock = chassis.lock().await;
                                    is_dt_inserted = chassis_lock.is_rack_inserted();
                                    drop(chassis_lock);
                                    sleep(LONG_SLEEP).await;
                                }
                            } else {
                                // put 10 mm for jumping over lane, tbd
                                navigation_computer.go_forward_slowly(10).await;
                            }
                        }

                        self.async_logger
                            .out_print(format!("{PRE_APPEND_STR} SI dau insert la rack"))
                            .await;
                    }
                    ActionType::RemoveRack { lane_number } => {
                        if !self.dock(&navigation_computer, &chassis).await {
                            continue;
                        }

                        let mut lane_nr = 0;
                        let mut arrived_at_desired_lane = false;
                        let mut chassis_lock: MutexGuard<'_, RealChassis>;
                        while !arrived_at_desired_lane {
                            navigation_computer.go_forward_slowly(100).await;
                            let mut arrived_at_a_lane = false;
                            while !arrived_at_a_lane {
                                chassis_lock = chassis.lock().await;
                                arrived_at_a_lane = chassis_lock.arrived_at_a_lane();
                                drop(chassis_lock);
                                sleep(SHORT_SLEEP).await;
                            }
                            navigation_computer.stop_moving(chassis.clone()).await;
                            chassis_lock = chassis.lock().await;
                            chassis_lock.stop_motors();
                            drop(chassis_lock);
                            lane_nr += 1;
                            lane_nr += 1;
                            if lane_nr == lane_number {
                                chassis_lock = chassis.lock().await;
                                chassis_lock.retrieve_rack();
                                drop(chassis_lock);
                                let mut is_dt_inserted = false;
                                while !is_dt_inserted {
                                    chassis_lock = chassis.lock().await;
                                    is_dt_inserted = chassis_lock.is_rack_inserted();
                                    drop(chassis_lock);
                                    sleep(LONG_SLEEP).await;
                                }
                                navigation_computer.undock(500).await;
                                arrived_at_desired_lane = true;
                            } else {
                                // put 10 mm for jumping over lane, tbd
                                navigation_computer.go_forward_slowly(10).await;
                            }
                        }

                        self.async_logger
                            .out_print(format!("{PRE_APPEND_STR} Si scot un rack"))
                            .await;
                    }
                    ActionType::BeerMe => todo!(),
                };

                self.async_logger
                    .out_print(format!("{PRE_APPEND_STR} Mission finished!"))
                    .await;

                if status_sender.send(MissionStatus::Completed).await.is_err() {
                    self.async_logger
                .err_print(format!(
                    "{PRE_APPEND_STR} Failed to send status update. MasterController might be down."
                ))
                .await;
                }

                keep_loop_going = *go.lock().await;
            }
            navigation_computer.stop_moving(chassis_clone.clone()).await;
            navigation_computer.kill().await;
            nav_computer_handle.await.unwrap();
            let mut chassis_lock = chassis_clone.lock().await;
            chassis_lock.stop_motors();
            chassis_lock.retrieve_rack();
            let mut is_dt_inserted = false;
            while !is_dt_inserted {
                is_dt_inserted = chassis_lock.is_rack_inserted();
                sleep(LONG_SLEEP).await;
            }
        })
    }

    async fn dock(
        self: &Arc<Self>,
        navigation_computer: &NavigationComputer,
        chassis: &Arc<sync::Mutex<RealChassis>>,
    ) -> bool {
        let mut chassis_lock = chassis.lock().await;
        chassis_lock.on_led();
        drop(chassis_lock);

        let relevement = match ElectricEye::find_marker() {
            Ok(r) => r,
            Err(_) => {
                let mut chassis_lock = chassis.lock().await;
                chassis_lock.off_led();
                drop(chassis_lock);

                return false;
            }
        };

        let mut chassis_lock = chassis.lock().await;
        chassis_lock.off_led();
        drop(chassis_lock);

        let mut chassis_lock: MutexGuard<'_, RealChassis>;
        navigation_computer.dock(relevement).await;
        let mut are_buttons_pressed = false;
        while !are_buttons_pressed {
            chassis_lock = chassis.lock().await;
            are_buttons_pressed = chassis_lock.are_buttons_pressed();
            drop(chassis_lock);
            sleep(SHORT_SLEEP).await;
        }
        navigation_computer.stop_moving(chassis.clone()).await;
        chassis_lock = chassis.lock().await;
        chassis_lock.stop_motors();
        drop(chassis_lock);

        true
    }
}
