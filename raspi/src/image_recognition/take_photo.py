from picamera2 import Picamera2
import time
from libcamera import Transform


picam2 = Picamera2()
config = picam2.create_still_configuration(
    transform=Transform(rotation=180), main={"size": (1000, 1000)}   # or Transform(hflip=1, vflip=1)
)
picam2.configure(config)

picam2.start()
time.sleep(2)

image = picam2.capture_array()
picam2.capture_file("/home/pi/Pictures/photo.jpg")
