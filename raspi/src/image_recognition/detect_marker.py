# import numpy as np 
# import cv2
# import math
# import sys
#
# def detect_yellow_marker(image_path):
#
#     real_height = 96.0
#     focal_length = 1200  # TODO compute camera calibration  
#
#     image = cv2.imread(image_path)
#     if image is None:
#         raise ValueError("Failed to load image.")
#     hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
#     lower_yellow = np.array([20, 100, 100])
#     upper_yellow = np.array([30, 255, 255])
#     mask = cv2.inRange(hsv, lower_yellow, upper_yellow)
#
#     contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
#
#     if contours:
#         c = max(contours, key=cv2.contourArea)
#         x, y, w, h = cv2.boundingRect(c)
#
#         image_height = h
#         estimated_distance = (real_height * focal_length) / image_height
#         #stripe = image[y:y+h, x:x+w]
#         #cv2.imwrite("detected_stripe100.jpg", stripe)
#         #print(f"Estimated distance to stripe: {estimated_distance:.2f} cm")
#
#         stripe_center_y = y + h / 2
#         image_center_y = image.shape[0] / 2
#         pixel_offset_y = stripe_center_y - image_center_y
#
#         angle_rad = math.atan(pixel_offset_y / focal_length)
#
#         angle_deg = math.degrees(angle_rad)
#         return estimated_distance, angle_deg
#     else:
#         raise ValueError("No yellow stripe detected.")
#
#     # cv2.imshow("Detected", image)
#     # cv2.waitKey(0)
#
#
# # detect_yellow_marker("C:\\Users\\ahrihorov\\Downloads\\test_yellow_stripe6.jpg")
#
# if __name__ == "__main__":
#     image_path = sys.argv[1]
#     try:
#         distance, angle = detect_yellow_marker(image_path)
#         print(f"{distance} {angle}")
#     except Exception as e:
#         print(f"ERROR: {e}")
#         sys.exit(1)    

# import numpy as np
# import cv2
# import math
# import sys
#
# def is_purple(hsv_pixel):
#     h, s, v = hsv_pixel
#     return (130 <= h <= 155) and (50 <= s <= 255) and (195 <= v <= 255)
#
# def is_yellow(hsv_pixel):
#     h, s, v = hsv_pixel
#     return (20 <= h <= 35) and (125 <= s <= 255) and (150 <= v <= 255)
#
# def vertical_color_transition_scan(image, step_x=1, min_stripe_height=10, max_stripe_height=200):
#     hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
#     h, w = hsv.shape[:2]
#     stripe_candidates = []
#
#     for x in range(0, w, step_x):
#         column = hsv[:, x, :]
#         transitions = []
#         for y in range(1, h):
#             current_color = column[y]
#             diff = np.linalg.norm(column[y].astype(np.float32) - column[y - 1].astype(np.float32))
#
#             if diff > 40:  
#                 print(current_color)
#                 transitions.append((y, current_color))
#
#         for i in range(len(transitions) - 2):
#             y1, color1 = transitions[i]
#             y2, color2 = transitions[i + 1]
#             y3, color3 = transitions[i + 2]
#
#             if is_purple(color1) and is_yellow(color2) and is_purple(color3):
#                 stripe_height = y3 - y1
#                 if min_stripe_height <= stripe_height <= max_stripe_height:
#                     stripe_candidates.append((x, y1, y3))
#
#     return stripe_candidates
#
#
# def estimate_distance_and_angle(stripe_regions, image_shape, real_stripe_height_cm=96.0, focal_length_px=1200):
#     h, w = image_shape[:2]
#
#     if not stripe_regions:
#         raise ValueError("No stripe-like regions detected.")
#
#     # Pick the best (e.g., most centered) stripe
#     best_stripe = max(stripe_regions, key=lambda s: s[2] - s[1])
#
#     x, y1, y3 = best_stripe
#     stripe_height = y3 - y1
#
#     distance_cm = (real_stripe_height_cm * focal_length_px) / stripe_height
#
#     stripe_center_y = (y1 + y3) / 2
#     image_center_y = h / 2
#     pixel_offset_y = stripe_center_y - image_center_y
#
#     angle_rad = math.atan(pixel_offset_y / focal_length_px)
#     angle_deg = math.degrees(angle_rad)
#
#     cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\detected_stripe2.jpg", best_stripe)
#     return distance_cm, angle_deg, best_stripe
#
#
# def detect_yellow_stripe_with_transitions(image_path):
#     real_stripe_height_cm = 96.0
#     focal_length_px = 1200
#
#     image = cv2.imread(image_path)
#     if image is None:
#         raise ValueError("Failed to load image.")
#
#     stripe_regions = vertical_color_transition_scan(image)
#
#     if not stripe_regions:
#         raise ValueError("No purple-yellow-purple transition detected.")
#
#     distance_cm, angle_deg, stripe = estimate_distance_and_angle(
#         stripe_regions, image.shape, real_stripe_height_cm, focal_length_px
#     )
#
#     # Optional: draw detected region
#     x, y1, y3 = stripe
#     cv2.rectangle(image, (x - 5, y1), (x + 5, y3), (0, 255, 0), 2)
#     # cv2.imshow("Detected Stripe", image)
#     # cv2.waitKey(0)
#
#     return distance_cm, angle_deg
#
#
# if __name__ == "__main__":
#     image_path = sys.argv[1]
#
#     try:
#         distance, angle = detect_yellow_stripe_with_transitions(image_path)
#         print(f"{distance:.2f} {angle:.2f}")
#     except Exception as e:
#         print(f"ERROR: {e}")
#         sys.exit(1)







import numpy as np 
import cv2
import math
import sys

def detect_yellow_marker(image_path):

    real_height = 96.0
    focal_length = 2296  # TODO compute camera calibration  

    image = cv2.imread(image_path)
    if image is None:
        raise ValueError("Failed to load image.")
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower_yellow = np.array([20, 100, 100])
    upper_yellow = np.array([35, 255, 255])
    lower_purple = np.array([130, 50, 195])
    upper_purple = np.array([155, 255, 255]) 
    mask_yellow = cv2.inRange(hsv, lower_yellow, upper_yellow)
    
    cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\mask_yellow2.jpg", mask_yellow)
    mask_purple = cv2.inRange(hsv, lower_purple, upper_purple)
    contours, _ = cv2.findContours(mask_yellow, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    if contours:
        for contour in contours:
            x, y, w, h = cv2.boundingRect(contour)
            if h/6 <= w:
                continue 
            left = image[y:y+h, x-9:x-1]
            right = image[y:y+h, x+w+1:x+w+9]
            mask_left = mask_purple[y:y+h, x-9:x-1]
            mask_right = mask_purple[y:y+h, x+w+1:x+w+9]
            result_left = cv2.bitwise_and(left, left, mask=mask_left)
            result_right = cv2.bitwise_and(right, right, mask=mask_right)
            if cv2.countNonZero(mask_left) > 0 and cv2.countNonZero(mask_right) > 0:
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
                stripe = image[y:y+h, x:x+w]
                cv2.imwrite("C:\\Users\\ahrihorov\\Downloads\\detected_stripe3.jpg", stripe)
                return estimated_distance, angle_deg

    return None, None 

    # if contours:
    #     c = max(contours, key=cv2.contourArea)
    #     x, y, w, h = cv2.boundingRect(c)
    #
    #     image_height = h
    #     estimated_distance = (real_height * focal_length) / image_height
    #     #stripe = image[y:y+h, x:x+w]
    #     #cv2.imwrite("detected_stripe100.jpg", stripe)
    #     #print(f"Estimated distance to stripe: {estimated_distance:.2f} cm")
    #
    #     stripe_center_y = y + h / 2
    #     image_center_y = image.shape[0] / 2
    #     pixel_offset_y = stripe_center_y - image_center_y
    #
    #     angle_rad = math.atan(pixel_offset_y / focal_length)
    #
    #     angle_deg = math.degrees(angle_rad)
    #     return estimated_distance, angle_deg
    # else:
    #     raise ValueError("No yellow stripe detected.")

    # cv2.imshow("Detected", image)
    # cv2.waitKey(0)


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

