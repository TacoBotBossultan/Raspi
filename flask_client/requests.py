from views import soseata_rust, connected
import json


def send_photo_request():
    # dăi
    req = {"Photo": None}
    req_ser = json.dumps(req)
    soseata_rust.sendall(req_ser.encode("utf-8"))

    # asteapta poza
    if connected:
        image_data = b""
        while True:
            chunk = soseata_rust.recv(4096)
            if not chunk:
                break
            image_data += chunk

        # save the received data to a file
        with open("received_image.jpg", "wb") as f:
            f.write(image_data)
        print("Image received and saved successfully.")
