use std::sync::Arc;

use crate::{
    mission_controller::missions::{ActionType, ExecutableMission, MissionStatus},
    navigation_computing::navigation_computer::NavigationComputer,
    utils::logging::AsyncLogger,
};
use tokio::sync::mpsc;

pub struct MissionController {
    mission_receiver: mpsc::Receiver<ExecutableMission>,
    status_sender: mpsc::Sender<MissionStatus>,
    async_logger: AsyncLogger,
    navigation_computer: Arc<NavigationComputer>,
}

static PRE_APPEND_STR: &str = "[Mission-Controller]";

impl MissionController {
    pub fn new(
        mission_receiver: mpsc::Receiver<ExecutableMission>,
        status_sender: mpsc::Sender<MissionStatus>,
        async_logger: AsyncLogger,
    ) -> Self {
        MissionController {
            mission_receiver,
            status_sender,
            async_logger,
            navigation_computer: Arc::new(NavigationComputer::new()),
        }
    }

    pub async fn run(mut self) {
        self.async_logger
            .out_print(format!("{PRE_APPEND_STR} Running..."))
            .await;
        while let Some(mission) = self.mission_receiver.recv().await {
            self.async_logger
                .out_print(format!(
                    "{PRE_APPEND_STR} Received new mission: {mission:#?}"
                ))
                .await;

            self.async_logger
                .out_print(format!("{PRE_APPEND_STR} Starting mission execution..."))
                .await;

            self.async_logger
                .out_print(format!("{PRE_APPEND_STR} Merg pe ruta:"))
                .await;

            self.async_logger
                .out_print(format!("{0:#?}", mission.route))
                .await;

            //TODO: trimiti pozitiile aalea catre nav computer
            for position in mission.route {}

            match mission.action {
                ActionType::GoToPosition => {
                    self.async_logger
                        .out_print(format!("{PRE_APPEND_STR} si atat lol"))
                        .await;
                }

                ActionType::TakePhoto => {
                    //TODO: I don't really get what this implies tbh
                    self.async_logger
                        .out_print(format!("{PRE_APPEND_STR} si fac si-o poza"))
                        .await;
                }

                ActionType::InsertRack => {
                    //TODO: tre sa-ti mai generezi aici pozitii suplimentare dupa care sa mergi ca
                    //sa faca pe bune actiunea
                    self.async_logger
                        .out_print(format!("{PRE_APPEND_STR} SI dau insert la rack"))
                        .await;
                }

                ActionType::RemoveRack => {
                    //TODO: tre sa-ti mai generezi aici pozitii suplimentare dupa care sa mergi ca
                    //sa faca pe bune actiunea
                    self.async_logger
                        .out_print(format!("{PRE_APPEND_STR} Si scot un rack"))
                        .await;
                }
            };

            self.async_logger
                .out_print(format!("{PRE_APPEND_STR} Mission finished!"))
                .await;

            if self
                .status_sender
                .send(MissionStatus::Completed)
                .await
                .is_err()
            {
                self.async_logger.err_print(format!("{PRE_APPEND_STR} Failed to send status update. MasterController might be down.")).await;
            }
        }
    }
}
