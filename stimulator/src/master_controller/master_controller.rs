use std::{fmt::format, format};

use crate::{
    Request_Response::{
        requests::Requests,
        responses::{self, Responses, RobotStates},
    },
    chassis::chassis_traits::Position,
    map_storage::route_storage::{MapStorage, RouteKey},
    mission_controller::missions::{ExecutableMission, MissionStatus},
    utils::logging::AsyncLogger,
};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Command {
    pub request: Requests,
    pub responder: oneshot::Sender<Responses>,
}

pub struct MasterController {
    command_receiver: mpsc::Receiver<Command>,
    mission_sender: mpsc::Sender<ExecutableMission>,
    status_receiver: mpsc::Receiver<MissionStatus>,
    robot_state: RobotStates,
    map_storage: MapStorage,
    async_logger: AsyncLogger,
}

static PRE_APPEND_STR: &str = "[Master-Controller]";

impl MasterController {
    pub fn new(
        command_receiver: mpsc::Receiver<Command>,
        mission_sender: mpsc::Sender<ExecutableMission>,
        status_receiver: mpsc::Receiver<MissionStatus>,
        async_logger: AsyncLogger,
    ) -> MasterController {
        MasterController {
            command_receiver,
            mission_sender,
            status_receiver,
            robot_state: RobotStates::Free,
            map_storage: MapStorage::new(),
            async_logger,
        }
    }

    pub async fn run(mut self) {
        self.async_logger
            .out_print(format!("{PRE_APPEND_STR} Running..."))
            .await;
        loop {
            tokio::select! {
                //  handle la commenzi de pe TCP server
                Some(cmd) = self.command_receiver.recv() => {
                    let response = self.handle_request(cmd.request).await;
                    if cmd.responder.send(response).is_err() {
                        self.async_logger.err_print(format!("{PRE_APPEND_STR:#?} Failed to send response to TCP handler.")
                        ).await;

                    }
                },

                // sauu status updates de la missioncontroller
                Some(status) = self.status_receiver.recv() => {
                    match status {
                        MissionStatus::Completed => {

                            self.async_logger.out_print(format!("{PRE_APPEND_STR} Primit status de Completed . Setting state to Free.")).await;

                            self.robot_state = RobotStates::Free;
                        },
                        MissionStatus::NotCompleted => {

                            self.async_logger.out_print(
                                format!("{PRE_APPEND_STR} Primit status de notComplete. Nu stiu cum ca nu trimite asta niciodata MissionControlleru lol")
                            ).await;

                            self.robot_state = RobotStates::Busy;
                        }
                    }
                },
            }
        }
    }

    async fn handle_request(&mut self, request: Requests) -> Responses {
        match request {
            Requests::State(_) => {
                // deocamdata sigur nu face nimic (mirel state gen lmao)
                let state_response = responses::StateResponse {
                    state: self.robot_state.clone(),
                };
                self.async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR} Aolo vrea asta sa stie ce face mission controlleru"
                    ))
                    .await;
                Responses::StateResponse(state_response)
            }

            Requests::Photo(_) => {
                // un vector random cu "datele de poza"
                //TODO: fa de aici direct poza
                let photo_response = responses::PhotoResponse {
                    photo_data: vec![0, 1, 2, 3, 4, 5], // Example photo data
                };

                self.async_logger.out_print(format!("{PRE_APPEND_STR} Aolo vrea asta o poza, da sa ma prefac ca virgula chiar am o camera, ii trimit poza asta")).await;
                Responses::PhotoResponse(photo_response)
            }

            Requests::DefineHome(home_coords) => {
                self.async_logger
                    .out_print(format!("{PRE_APPEND_STR} Defining home: "))
                    .await;
                self.map_storage.store_position(
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
                self.async_logger
                    .out_print(format!("{PRE_APPEND_STR} Storeuieste Routa"))
                    .await;

                match self.map_storage.store_route(&stored_route) {
                    Ok(res) => {
                        self.async_logger.out_print(format!("{PRE_APPEND_STR} Am reusit sa storuiesc ruta si am primit mesaju: {res:#?}")).await;
                    }

                    Err(errmsg) => {
                        let error_msg =
                            format!("{PRE_APPEND_STR} Eroare cand am storuit ruta: {errmsg:#?}");
                        self.async_logger.err_print(error_msg.clone()).await;

                        return Responses::GeneralResponse(responses::GeneralResponse {
                            status: 69,
                            message: error_msg,
                        });
                    }
                }

                self.async_logger
                    .out_print("Am storuit urmatoarea ruta:".to_string())
                    .await;

                let route_key = RouteKey::new(
                    stored_route.get_start_position_name(),
                    stored_route.get_destination_position_name(),
                );

                let route = self.map_storage.get_route(&route_key);

                self.async_logger.out_print(format!("{route:#?}")).await;

                Responses::GeneralResponse(responses::GeneralResponse {
                    status: 200,
                    message: "OK".to_string(),
                })
            }

            Requests::MissionRequest(mission) => {
                self.async_logger
                    .out_print(format!(
                        "{PRE_APPEND_STR} Se incepe missiunea: {mission:#?}"
                    ))
                    .await;

                if self.robot_state == RobotStates::Busy {
                    return Responses::GeneralResponse(responses::GeneralResponse {
                        status: 400,
                        message: "Robotu e ocupat acum, nu poa sa ia niciun request saracu"
                            .to_string(),
                    });
                }

                let route = self.map_storage.get_route(&mission.route).unwrap();
                let executable_mission = ExecutableMission::new(mission.action, route);
                if self.mission_sender.send(executable_mission).await.is_err() {
                    self.async_logger.err_print(format!(
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
