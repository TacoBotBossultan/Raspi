use core::result::Result;
use std::process::{Command, Stdio};

use crate::Request_Response::responses::Photo;

struct LeRelevement {
    distance: u32,
    angle: u32,
}

impl LeRelevement {
    pub fn new(distance: u32, angle: u32) -> Self {
        LeRelevement { distance, angle }
    }
}

struct ElectricEye;

impl ElectricEye {
    pub fn new() -> Self {
        ElectricEye
    }

    pub fn take_photo() -> Vec<u8> {}

    // Atentie, image e un vector aici, python are nevoie de un path.
    pub fn find_marker() -> Result<LeRelevement, String> {
        let image = ElectricEye::take_photo();

        let output = Command::new("python")
            .arg("detect_marker.py")
            .arg(image)
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| "Failed to run Python script: ".to_string() + &e.to_string())?;

        if !output.status.success() {
            return Err("Python error".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let values: Vec<&str> = stdout.trim().split_whitespace().collect();

        if values.len() != 2 {
            return Err("Expected 2 values".to_string());
        }

        let distance = values[0]
            .parse::<f64>()
            .map_err(|e| "Failed to parse distance: ".to_string() + &e.to_string())?;
        let angle = values[1]
            .parse::<f64>()
            .map_err(|e| "Failed to parse angle: ".to_string() + &e.to_string())?;
        let relevement = LeRelevement::new(distance, angle);
        Ok(relevement)
    }
}
