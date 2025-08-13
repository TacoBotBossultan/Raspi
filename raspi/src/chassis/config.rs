use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub utilitiesp_proccessor_id: String,
    pub drivesp_proccessor_id: String,
}
