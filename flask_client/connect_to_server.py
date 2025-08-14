import socket
import json

HOST = "127.0.0.1"
PORT = 8080


def send_request(sock, request_data):
    try:
        message = json.dumps(request_data).encode("utf-8")
        print(message)

        print(f"\n[CLIENT] Trimitem: {request_data}")
        sock.sendall(message)

        response_bytes = sock.recv(1024)
        if not response_bytes:
            print("[CLIENT] Serverul a inchid conexiunea.")
            return None

        response_data = json.loads(response_bytes.decode("utf-8"))
        print(f"[SERVER] Raspsuns cu : {response_data}")
        return response_data

    except json.JSONDecodeError as e:
        print(f"[ERROR] N-am putut decoda respunsul de json: {e}")
        print(f"Raspunsu : {response_bytes.decode('utf-8')}")
        return None
    except Exception as e:
        print(f"[ERROR] EROAREE!: {e}")
        return None


# socket, daca s-a conectat
def connect_to_server() -> tuple[socket.socket, bool]:
    soseata = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        soseata.connect((HOST, PORT))
        return (soseata, True)
    except ConnectionRefusedError:
        print("[ERROR] Nu s-a conectat la rustu ala jegos. sigur e portnit serveru?")
        return (soseata, False)
    except Exception as e:
        print(f"[ERROR] alt fel de eroare: {e}")
        return (soseata, False)
