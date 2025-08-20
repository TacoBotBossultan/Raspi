import socket
import json
import time

HOST = '127.0.0.1'  
PORT = 8080        

def send_request(sock, request_data):
    try:
        message = json.dumps(request_data).encode('utf-8')
        print(message)
        
        print(f"\n[CLIENT] Trimitem: {request_data}")
        sock.sendall(message)

        response_bytes = sock.recv(1024)
        if not response_bytes:
            print("[CLIENT] Serverul a inchid conexiunea.")
            return None
            
        response_data = json.loads(response_bytes.decode('utf-8'))
        print(f"[SERVER] Raspsuns cu : {response_data}")
        return response_data

    except json.JSONDecodeError as e:
        print(f"[ERROR] N-am putut decoda respunsul de json: {e}")
        #print(f"Raspunsu : {response_bytes.decode('utf-8')}")
        return None
    except Exception as e:
        print(f"[ERROR] EROAREE!: {e}")
        return None

def main():
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            print(f"Ne conectam la {HOST}:{PORT}...")
            s.connect((HOST, PORT))
            print("CONNECTION SUCCESSFUL!!")

            # test 1: in ce state esti?
            # `Requests::State(State)`
            state_request = {"State": None}
            send_request(s, state_request)
            time.sleep(1) 

            # test 2: poza?
            # `Requests::Photo(Photo)`
            photo_request = {"Photo": None}
            send_request(s, photo_request)
            time.sleep(1)

            #test 3: aici iti e casa
            #`Requests::DefineHome(DefineHome)`
            define_home_request = {
                "DefineHome": {
                    "home_x": 200,
                    "home_y": 200,
                    "home_theta": 90
                }
            }
            send_request(s, define_home_request)
            time.sleep(1)

            # test 4: dam store la o ruta
            #  `Requests::StoreRoute(StoreRoute)`
            store_route_request = {
                "StoreRoute": {
                    "start_position_name": "Home",
                    "route": [
                          { "direction_type" :  "Forward"  ,  "value" : 100  } ,
                          { "direction_type" :  "Right"  ,  "value" : 100  } ,
                          { "direction_type" :  "RotateLeft"  ,  "value" : 90} ,
                        ],
                    "destination_position_name": "acolo"
                }
            }

            send_request(s, store_route_request)
            time.sleep(1)

            # test 5: incepe o misiune noua
            # `Requests::StartMission(StartMission)`
            start_mission_request = {
                "MissionRequest": {
                    "action": "TakePhoto",
                    "route" : {"start_name" : "Home", "destination_name" : "acolo"}
                }
            }
            send_request(s, start_mission_request)
            time.sleep(1)

    except ConnectionRefusedError:
        print("[ERROR] Nu s-a conectat. sigur e portnit serveru?")
    except Exception as e:
        print(f"[ERROR] alt fel de eroare: {e}")

    print("\nGATAA TOTT.")


if __name__ == '__main__':
    main()

