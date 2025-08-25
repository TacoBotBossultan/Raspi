use core::result::Result;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyBytesMethods;
use pyo3::types::{PyBytes, PyDict};

#[derive(Debug, Clone, Copy)]
pub struct LeRelevement {
    distance: f32,
    angle: f32,
}

impl LeRelevement {
    pub fn new(distance: f32, angle: f32) -> Self {
        LeRelevement { distance, angle }
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElectricEye;

impl ElectricEye {
    pub fn new() -> Self {
        ElectricEye
    }

    pub fn take_photo() -> PyResult<Vec<u8>> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);

            py.run(
                c_str!(
                    r#"
from picamera2 import Picamera2
import time
from libcamera import Transform
import cv2
import numpy as np

picam2 = Picamera2()
config = picam2.create_still_configuration(
    transform=Transform(rotation=180), main={"size": (1000, 1000), "format": "RGB888"}
)
picam2.configure(config)
picam2.start()
time.sleep(2)

image = picam2.capture_array()
_, buffer = cv2.imencode('.jpg', image)
jpg_bytes = buffer.tobytes()
picam2.close()
"#
                ),
                None,
                Some(&locals),
            )?;

            let any = locals
                .get_item("jpg_bytes")?
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("jpg_bytes not found"))?;

            let pybytes: pyo3::Bound<'_, PyBytes> = any.extract::<pyo3::Bound<'_, PyBytes>>()?;
            Ok(pybytes.as_bytes().to_vec())
        })
    }

    pub fn find_marker() -> Result<LeRelevement, String> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            if let Err(error) = py.run(
                c_str!(
                    r#"
    import numpy as np 
    import cv2
    import math
    import sys
    from picamera2 import Picamera2
    import time
    from libcamera import Transform

    real_height = 16.5
    focal_length = 788.78    
    picam2 = Picamera2()
    config = picam2.create_still_configuration(
        transform=Transform(rotation=180), main={"size": (1000, 1000)}   # or Transform(hflip=1, vflip=1)
    )
    picam2.configure(config)

    picam2.start()
    time.sleep(2)

    image = picam2.capture_array()
    picam2.capture_file("/home/pi/Pictures/photo.jpg")

    if image is None:
        raise ValueError("Failed to load image.")
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower_yellow = np.array([20, 100, 100])
    upper_yellow = np.array([35, 255, 255])
    lower_purple = np.array([100, 60, 30])
    upper_purple = np.array([160, 255, 255]) 
    mask_yellow = cv2.inRange(hsv, lower_yellow, upper_yellow)
    
    mask_purple = cv2.inRange(hsv, lower_purple, upper_purple)
    contours, _ = cv2.findContours(mask_yellow, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    if contours:
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            print(x, y, w, h)
            if h/2 <= w:
                continue 
            mask_left = mask_purple[y:y+h, x-9:x-1]
            mask_right = mask_purple[y:y+h, x+w+1:x+w+9]
            if cv2.countNonZero(mask_left) > 0 and cv2.countNonZero(mask_right) > 0:
                image_height = h
                estimated_distance = (real_height * focal_length) / image_height

                stripe_center_x = x + w / 2
                image_center_x = image.shape[1] / 2
                pixel_offset_x = stripe_center_x - image_center_x
                angle_rad = math.atan(pixel_offset_x / focal_length)
                angle_deg = math.degrees(angle_rad)               
"#
                ),
                None,
                Some(&locals),
            ) {
                error.display(py);
                return Err(error.to_string());
            }
            let angle_deg: f32 = match locals
                .get_item("angle_deg")
                .unwrap()
                .unwrap()
                .extract::<f32>()
            {
                Ok(angle) => angle,
                Err(_) => return Err("Invalid value for angle".to_string()),
            };

            let estimated_distance: f32 = match locals
                .get_item("estimated_distance")
                .unwrap()
                .unwrap()
                .extract::<f32>()
            {
                Ok(distance) => distance,
                Err(_) => return Err("Invalid value for distance".to_string()),
            };

            Ok(LeRelevement::new(estimated_distance, angle_deg))
        })
    }
}

impl Default for ElectricEye {
    fn default() -> Self {
        Self::new()
    }
}
