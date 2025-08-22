use std::{format, sync::Arc};

use crate::{
    chassis::{
        self,
        chassis_traits::{ChassisTraits, Position},
        real_chassis::RealChassis,
    },
    image_recognition::electric_eye::ElectricEye,
    map_storage::route_storage::{MapStorage, RouteKey},
    mission_controller::{
        mission_controller::MissionController,
        missions::{ExecutableMission, MissionStatus},
    },
    navigation_computing::navigation_computer::NavigationComputer,
    request_response::{
        requests::Requests,
        responses::{self, Responses, RobotStates},
    },
    utils::logging::AsyncLogger,
};
use tokio::{
    spawn,
    sync::{self, Notify, mpsc, oneshot},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct Command {
    pub request: Requests,
    pub responder: oneshot::Sender<Responses>,
}

pub struct MasterController {
    robot_state: sync::Mutex<RobotStates>,
    breaker: Arc<Notify>,
}

static PRE_APPEND_STR: &str = "[Master-Controller]";

impl MasterController {
    pub fn stop(&self) {
        self.breaker.notify_one();
    }

    pub fn new() -> MasterController {
        let breaker = Arc::new(Notify::new());
        let robot_state = sync::Mutex::new(RobotStates::Free);
        MasterController {
            robot_state,
            breaker,
        }
    }

    pub async fn run(
        self: Arc<Self>,
        controller: MissionController,
        navigation_computer: Arc<NavigationComputer>,
        chassis: Arc<sync::Mutex<RealChassis>>,
        mut command_receiver: mpsc::Receiver<Command>,
        mut map_storage: MapStorage,
        async_logger: AsyncLogger,
    ) -> JoinHandle<()> {
        async_logger
            .out_print(format!("{PRE_APPEND_STR} Running..."))
            .await;

        let breaker_clone = self.breaker.clone();
        spawn(async move {
            let (mission_sender, mission_receiver) = mpsc::channel::<ExecutableMission>(2);
            let (status_sender, mut status_receiver) = mpsc::channel::<MissionStatus>(2);
            let mission_controller = Arc::new(controller);
            let join_handle = mission_controller
                .clone()
                .run(
                    mission_receiver,
                    status_sender,
                    Arc::clone(&navigation_computer),
                    Arc::clone(&chassis),
                )
                .await;
            loop {
                tokio::select! {
                    //  handle la commenzi de pe TCP server
                    Some(cmd) = command_receiver.recv() => {
                        let response = MasterController::handle_request(cmd.request, &mission_sender, &mut map_storage, &async_logger, self.robot_state.lock().await.clone(), Arc::clone(&chassis),Arc::clone(&navigation_computer)).await;
                        if cmd.responder.send(response).is_err() {
                            async_logger.err_print(format!("{PRE_APPEND_STR:#?} Failed to send response to TCP handler.")
                            ).await;

                        }
                    },

                    // sauu status updates de la missioncontroller
                    Some(status) = status_receiver.recv() => {
                        match status {
                            MissionStatus::Completed => {

                                async_logger.out_print(format!("{PRE_APPEND_STR} Primit status de Completed . Setting state to Free.")).await;

                                *self.robot_state.lock().await =  RobotStates::Free;
                            },
                            MissionStatus::NotCompleted => {

                                async_logger.out_print(
                                    format!("{PRE_APPEND_STR} Primit status de notComplete. Nu stiu cum ca nu trimite asta niciodata MissionControlleru lol")
                                ).await;

                                *self.robot_state.lock().await = RobotStates::Busy;
                            }
                        }
                    },

                    _ = breaker_clone.notified() => {
                        mission_controller.stop().await;
                        join_handle.await.unwrap();
                        break;
                    }
                }
            }
        })
    }

    async fn handle_request(
        request: Requests,
        mission_sender: &mpsc::Sender<ExecutableMission>,
        map_storage: &mut MapStorage,
        async_logger: &AsyncLogger,
        robot_state: RobotStates,
        chassis: Arc<sync::Mutex<RealChassis>>,
        nav_computer: Arc<NavigationComputer>,
    ) -> Responses {
        match request {
            Requests::State(_) => {
                // deocamdata sigur nu face nimic (mirel state gen lmao)
                let state_response = responses::StateResponse { state: robot_state };
                async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR} Aolo vrea asta sa stie ce face mission controlleru"
                    ))
                    .await;
                Responses::StateResponse(state_response)
            }

            Requests::Photo(_) => {
                // un vector random cu "datele de poza"
                //TODO: fa de aici direct poza
                async_logger
                    .out_print(format!("{PRE_APPEND_STR} Aolo vrea asta o poza"))
                    .await;
                let mut chassis_lock = chassis.lock().await;
                chassis_lock.on_led();
                drop(chassis_lock);
                let photo = ElectricEye::take_photo();
                let photo_response = responses::PhotoResponse {
                    photo_data: photo.unwrap(), // Example photo data
                };
                let mut chassis_lock = chassis.lock().await;
                chassis_lock.off_led();
                drop(chassis_lock);

                Responses::PhotoResponse(photo_response)
            }

            Requests::DefineHome(home_coords) => {
                async_logger
                    .out_print(format!("{PRE_APPEND_STR} Defining home: "))
                    .await;
                map_storage.store_position(
                    Position::create(
                        Some("Home".to_string()),
                        home_coords.get_x(),
                        home_coords.get_y(),
                        home_coords.get_theta(),
                    )
                    .unwrap(),
                );
                Responses::GeneralResponse(responses::GeneralResponse {
                    status: 200,
                    message: "OK".to_string(),
                })
            }
            Requests::StoreRoute(stored_route) => {
                async_logger
                    .out_print(format!("{PRE_APPEND_STR} Storeuieste Routa"))
                    .await;

                match map_storage.store_route(&stored_route) {
                    Ok(res) => {
                        async_logger.out_print(format!("{PRE_APPEND_STR} Am reusit sa storuiesc ruta si am primit mesaju: {res:#?}")).await;
                    }

                    Err(errmsg) => {
                        let error_msg =
                            format!("{PRE_APPEND_STR} Eroare cand am storuit ruta: {errmsg:#?}");
                        async_logger.err_print(error_msg.clone()).await;

                        return Responses::GeneralResponse(responses::GeneralResponse {
                            status: 69,
                            message: error_msg,
                        });
                    }
                }

                async_logger
                    .out_print("Am storuit urmatoarea ruta:".to_string())
                    .await;

                let route_key = RouteKey::new(
                    stored_route.get_start_position_name(),
                    stored_route.get_destination_position_name(),
                );

                let route = map_storage.get_route(&route_key);

                async_logger.out_print(format!("{route:#?}")).await;

                Responses::GeneralResponse(responses::GeneralResponse {
                    status: 200,
                    message: "OK".to_string(),
                })
            }

            Requests::MissionRequest(mission) => {
                async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR} Se incepe missiunea: {mission:#?}"
                    ))
                    .await;

                if robot_state == RobotStates::Busy {
                    return Responses::GeneralResponse(responses::GeneralResponse {
                        status: 400,
                        message: "Robotu e ocupat acum, nu poa sa ia niciun request saracu"
                            .to_string(),
                    });
                }

                let route = map_storage.get_route(&mission.route).unwrap();
                let executable_mission = ExecutableMission::new(mission.action, route);
                if mission_sender.send(executable_mission).await.is_err() {
                    async_logger.err_print(format!(
                            "{PRE_APPEND_STR:#?} eroare la trimiterea misiunii catre mission controller"
                        )).await;
                }

                Responses::GeneralResponse(responses::GeneralResponse {
                    status: 200,
                    message: "OK".to_string(),
                })
            }
        }
    }
}

impl Default for MasterController {
    fn default() -> Self {
        Self::new()
    }
}
