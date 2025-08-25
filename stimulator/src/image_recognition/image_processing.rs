extern crate pyo3;
use pyo3::Py;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::process::{Command, Stdio};

fn process_image() -> PyResult<()> {
    let python_code = r#"import numpy as np 
import cv2
import math
import sys
    
    real_height = 96.0
    focal_length = 1200  # TODO compute camera calibration  

    image = cv2.imread("C:\\Users\\ahrihorov\\Downloads\\test_yellow_stripe6.jpg")
    if image is None:
        raise ValueError("Failed to load image.")
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower_yellow = np.array([20, 100, 100])
    upper_yellow = np.array([30, 255, 255])
    mask = cv2.inRange(hsv, lower_yellow, upper_yellow)

    contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    if contours:
        c = max(contours, key=cv2.contourArea)
        x, y, w, h = cv2.boundingRect(c)
    
        image_height = h
        estimated_distance = (real_height * focal_length) / image_height
        #stripe = image[y:y+h, x:x+w]
        #cv2.imwrite("detected_stripe100.jpg", stripe)
        #print(f"Estimated distance to stripe: {estimated_distance:.2f} cm")
        
        stripe_center_y = y + h / 2
        image_center_y = image.shape[0] / 2
        pixel_offset_y = stripe_center_y - image_center_y

        angle_rad = math.atan(pixel_offset_y / focal_length)
        
        angle_deg = math.degrees(angle_rad)
        return estimated_distance, angle_deg
    else:
        raise ValueError("No yellow stripe detected.")
"#;
    Python::with_gil(|py| {
        py.run(python_code, None, None)?;
        Ok(())
    })
}

fn main() {
    match process_image() {
        Ok((distance, angle)) => {
            println!("Distance to stripe: {:.2} cm", distance);
            println!("Angle to stripe: {:.2} degrees", angle);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
