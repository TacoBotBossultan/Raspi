import numpy as np 
import cv2
import math
import sys

def detect_yellow_marker(image_path):

    real_height = 96.0
    focal_length = 1200  # TODO compute camera calibration  

    image = cv2.imread(image_path)
    if image is None:
        raise ValueError("Failed to load image.")
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower_yellow = np.array([20, 100, 100])
    upper_yellow = np.array([35, 255, 255])
    lower_purple = np.array([125, 50, 50])
    upper_purple = np.array([160, 255, 255]) 
    mask_yellow = cv2.inRange(hsv, lower_yellow, upper_yellow)
    
    cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\mask_yellow2.jpg", mask_yellow)
    mask_purple = cv2.inRange(hsv, lower_purple, upper_purple)
    cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\mask_purple2.jpg", mask_purple)
    contours, _ = cv2.findContours(mask_yellow, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    if contours:
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            if h/6 <= w:
                continue 
            mask_left = mask_purple[y:y+h, x-9:x-1]
            mask_right = mask_purple[y:y+h, x+w+1:x+w+9]
            if cv2.countNonZero(mask_left) > 0 and cv2.countNonZero(mask_right) > 0:
                image_height = h
                estimated_distance = (real_height * focal_length) / image_height

                stripe_center_y = y + h / 2
                image_center_y = image.shape[0] / 2
                pixel_offset_y = stripe_center_y - image_center_y

                angle_rad = math.atan(pixel_offset_y / focal_length)

                angle_deg = math.degrees(angle_rad)
                stripe = image[y:y+h, x:x+w]
                cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\detected_stripe3.jpg", stripe)
                return estimated_distance, angle_deg

    return None, None 
    
# detect_yellow_marker("C:\\Users\\ahrihorov\\Downloads\\test_yellow_stripe6.jpg")

if __name__ == "__main__":
    image_path = sys.argv[1]
    try:
        distance, angle = detect_yellow_marker(image_path)
        if distance is None:
            raise ValueError("No yellow stripe detected.")
        print(f"{distance} {angle}")
    except Exception as e:
        print(f"ERROR: {e}")
        sys.exit(1)    

