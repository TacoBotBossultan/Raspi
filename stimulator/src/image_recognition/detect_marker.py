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

    # cv2.imshow("Detected", image)
    # cv2.waitKey(0)


# detect_yellow_marker("C:\\Users\\ahrihorov\\Downloads\\test_yellow_stripe6.jpg")

if __name__ == "__main__":
    image_path = sys.argv[1]
    try:
        distance, angle = detect_yellow_marker(image_path)
        print(f"{distance} {angle}")
    except Exception as e:
        print(f"ERROR: {e}")
        sys.exit(1)    
