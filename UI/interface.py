import tkinter as tk
from tkinter import ttk
from tkinter import messagebox
import socket
import time
import json
import os

HOST = "127.0.0.1"
PORT = 8080


def send_photo_request_and_save_photo(sock):
    req = {"Photo": None}
    req_ser = json.dumps(req)

    photo_path = "~/Raspi_Official/UI/received_image.jpg"

    if os.path.exists(photo_path):
        os.remove(photo_path)
    else:
        print("No need to delete the file its fine")
    print("Vor astia poza...")
    sock.sendall(req_ser.encode("utf-8"))

    cnt = 0
    image_data = b""
    while True:
        try:
            chunk = sock.recv(4096)
            cnt += 1
            print("suntem la loopu din recv:", cnt)
            print("Chunku e:", chunk)
            image_data += chunk
            if b"}}" in chunk:
                print("gataaa s-ar terminat nu mai am primit nimic, dupa:", cnt)
                break
        except socket.timeout:
            print("am asteptat mai mult de 5 secundici")
            break

    response_data = json.loads(image_data.decode("utf-8"))
    # print('Response Data cu poza:' ,response_data)
    # save the received data to a file
    with open("received_image.jpg", "wb") as f:
        f.write(bytes(response_data["PhotoResponse"]["photo_data"]))
    print("Image received and saved successfully.")


def send_general_request(sock, request_data, receive_size) -> dict | None:
    try:
        message = json.dumps(request_data).encode("utf-8")
        print(message)

        print(f"\n[CLIENT] Trimitem: {request_data}")
        sock.sendall(message)

        response_bytes = sock.recv(receive_size)

        print("Response bytes: ", response_bytes)
        if not response_bytes:
            print("[CLIENT] Serverul a inchid conexiunea.")
            return None

        response_data = json.loads(response_bytes.decode("utf-8"))
        # print(f"[SERVER] Raspsuns cu : {response_data}")
        return response_data

    except json.JSONDecodeError as e:
        print(f"[ERROR] N-am putut decoda respunsul de json: {e}")
        # print(f"Raspunsu : {response_bytes.decode('utf-8')}")
        return None
    except Exception as e:
        print(f"[ERROR] EROAREE!: {e}")
        return None


s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.settimeout(5)


class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.conn = self.connect_to_server()
        if not self.conn:
            self.destroy()
            return

        self.title("TacoBot user interface")
        self.geometry("1000x700")

        self.container = tk.Frame(self)
        self.container.pack(fill="both", expand=True)

        self.pages = {}

        for P in (
            DefineHomePage,
            StoreRoutePage,
            GoAndTakePhotoPage,
            TakePhotoPage,
            GoToPositionPage,
            InsertRackPage,
            RemoveRackPage,
        ):
            page_name = P.__name__
            frame = P(parent=self.container, controller=self)
            self.pages[page_name] = frame
            frame.grid(row=0, column=0, sticky="nsew")

        nav_frame = tk.Frame(self)
        nav_frame.pack(side="top", fill="x")

        define_home_button = tk.Button(
            nav_frame,
            text="Define home",
            command=lambda: self.show_page("DefineHomePage"),
        )
        store_route_button = tk.Button(
            nav_frame,
            text="Store route",
            command=lambda: self.show_page("StoreRoutePage"),
        )
        take_photo_button = tk.Button(
            nav_frame,
            text="Take photo",
            command=lambda: self.show_page("TakePhotoPage"),
        )
        go_and_take_photo_button = tk.Button(
            nav_frame,
            text="Go and take photo",
            command=lambda: self.show_page("GoAndTakePhotoPage"),
        )
        go_to_position_button = tk.Button(
            nav_frame,
            text="Go to position",
            command=lambda: self.show_page("GoToPositionPage"),
        )
        insert_rack_button = tk.Button(
            nav_frame,
            text="Insert Rack",
            command=lambda: self.show_page("InsertRackPage"),
        )
        remove_rack_button = tk.Button(
            nav_frame,
            text="Remove Rack",
            command=lambda: self.show_page("RemoveRackPage"),
        )

        define_home_button.pack(side="left", expand=True, fill="x")
        store_route_button.pack(side="left", expand=True, fill="x")
        take_photo_button.pack(side="left", expand=True, fill="x")
        go_and_take_photo_button.pack(side="left", expand=True, fill="x")
        go_to_position_button.pack(side="left", expand=True, fill="x")
        insert_rack_button.pack(side="left", expand=True, fill="x")
        remove_rack_button.pack(side="left", expand=True, fill="x")

        self.show_page("DefineHomePage")

    def show_page(self, page_name):
        page = self.pages[page_name]
        page.tkraise()

    def connect_to_server(self):
        try:
            print(f"Ne conectam la {HOST}:{PORT}...")
            s.connect((HOST, PORT))
            print("CONNECTION SUCCESSFUL!!")
            return s

        except ConnectionRefusedError:
            messagebox.showerror("Error", "Connection refused! Are you dumb?")
            return None

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")
            return None


class DefineHomePage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        self.x_label = tk.Label(self, text="x coordinate:", font=("Arial", 12))
        self.y_label = tk.Label(self, text="y coordinate:", font=("Arial", 12))
        self.theta_label = tk.Label(self, text="theta:", font=("Arial", 12))

        self.x_entry = tk.Entry(self, width=30)
        self.y_entry = tk.Entry(self, width=30)
        self.theta_entry = tk.Entry(self, width=30)

        self.x_label.pack(padx=10, pady=(10, 0))
        self.x_entry.pack(padx=10, pady=5)
        self.y_label.pack(padx=10, pady=(10, 0))
        self.y_entry.pack(padx=10, pady=5)
        self.theta_label.pack(padx=10, pady=(10, 0))
        self.theta_entry.pack(padx=10, pady=5)

        self.result_label = tk.Label(self, text="", font=("Arial", 12))
        self.result_label.pack(pady=10)

        define_home_btn = tk.Button(self, text="Submit", command=self.on_define_home)
        cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        define_home_btn.pack(side="left", padx=20, pady=10)
        cancel_btn.pack(side="left", padx=20, pady=10)

    def on_define_home(self):
        try:
            x_text = int(self.x_entry.get())
            y_text = int(self.y_entry.get())
            theta_text = int(self.theta_entry.get())
            coordinates_dict = {
                "home_x": x_text,
                "home_y": y_text,
                "home_theta": theta_text,
            }
            define_home_request = {"DefineHome": coordinates_dict}
            send_general_request(s, define_home_request, 1024)
            time.sleep(1)

            self.result_label.config(text="✅ You defined the home")

        except ValueError:
            messagebox.showerror("Error", "The coordinates should be integers!")

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

    def on_cancel(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

        self.result_label.config(text="")


class StoreRoutePage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        tk.Label(self, text="Starting position:").grid(
            row=0, column=0, padx=10, pady=5, sticky="e"
        )
        self.starting_position_field = tk.Entry(self, width=30)
        self.starting_position_field.grid(row=0, column=1, padx=10, pady=5)

        tk.Label(self, text="Destination position:").grid(
            row=1, column=0, padx=10, pady=5, sticky="e"
        )
        self.destination_position_field = tk.Entry(self, width=30)
        self.destination_position_field.grid(row=1, column=1, padx=10, pady=5)

        self.define_route_btn = tk.Button(
            self, text="Define route", command=self.show_more_fields
        )
        self.define_route_btn.grid(row=2, column=0, columnspan=2, pady=10)

        self.direction_label = None
        self.direction_entry = None
        self.value_label = None
        self.value_entry = None
        self.submit_btn = None
        self.stop_btn = None

        self.submitted_texts = []

        self.summary_label = tk.Label(self, text="", justify="left", font=("Arial", 12))

    def show_more_fields(self):
        if not self.direction_label:
            self.direction_label = tk.Label(self, text="Direction:")
            self.direction_label.grid(row=3, column=0, padx=10, pady=5, sticky="e")

            self.direction_entry = ttk.Combobox(
                self,
                values=[
                    "Forward",
                    "Backward",
                    "Right",
                    "Left",
                    "Rotate Right",
                    "Rotate Left",
                ],
            )
            self.direction_entry.set("Choose a direction...")
            self.direction_entry.grid(row=3, column=1, padx=10, pady=5)

            self.value_label = tk.Label(self, text="Value (in mm):")
            self.value_label.grid(row=4, column=0, padx=10, pady=5, sticky="e")
            self.value_entry = tk.Entry(self, width=30)
            self.value_entry.grid(row=4, column=1, padx=10, pady=5)

            self.submit_btn = tk.Button(self, text="Submit", command=self.on_submit)
            self.submit_btn.grid(row=5, column=0, padx=10, pady=10)

            self.stop_btn = tk.Button(self, text="Stop", command=self.on_stop)
            self.stop_btn.grid(row=5, column=1, padx=10, pady=10)

    def on_submit(self):
        if self.direction_entry and self.value_entry:
            option = self.direction_entry.get()
            text = self.value_entry.get()
            if option or text:
                self.submitted_texts.append((option, text))
            self.value_entry.delete(0, tk.END)

    def on_stop(self):
        if self.direction_label:
            self.direction_label.destroy()
            self.direction_label = None
        if self.direction_entry:
            self.direction_entry.destroy()
            self.direction_entry = None
        if self.value_label:
            self.value_label.destroy()
            self.value_label = None
        if self.value_entry:
            self.value_entry.destroy()
            self.value_entry = None
        if self.submit_btn:
            self.submit_btn.destroy()
            self.submit_btn = None
        if self.stop_btn:
            self.stop_btn.destroy()
            self.stop_btn = None

        starting_position_text = self.starting_position_field.get()
        destination_position_text = self.destination_position_field.get()
        summary = f"Summary:\n\n\tStarting position: {starting_position_text}\n\tDestination position: {destination_position_text}\n\n\tRoute:\n"
        for i, (opt, txt) in enumerate(self.submitted_texts, start=1):
            summary += f"  \t\tRoute step {i}: {opt} | {txt}\n"

        self.summary_label.config(text=summary)
        self.summary_label.grid(row=6, column=0, columnspan=2, pady=10)

        self.cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        self.cancel_btn.grid(row=7, column=0, pady=10)
        self.save_btn = tk.Button(self, text="Save", command=self.on_save)
        self.save_btn.grid(row=7, column=1, pady=10)

    def on_cancel(self):
        self.starting_position_field.delete(0, tk.END)
        self.destination_position_field.delete(0, tk.END)
        self.submitted_texts.clear()
        self.summary_label.config(text="")
        self.summary_label.grid_forget()
        self.cancel_btn.grid_forget()
        self.save_btn.grid_forget()

    def on_save(self):
        try:
            starting_position_text = self.starting_position_field.get()
            destination_position_text = self.destination_position_field.get()
            route_list = []
            for i, (opt, txt) in enumerate(self.submitted_texts, start=1):
                current_dict = {"direction_type": opt, "value": txt}
                route_list.append(current_dict)
            route_dict = {
                "starting_position_name": starting_position_text,
                "route": route_list,
                "destination_position_name": destination_position_text,
            }
            store_route_request = {"StoreRoute": route_dict}

            send_general_request(s, store_route_request, 1024)
            time.sleep(1)

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.on_cancel()


class GoAndTakePhotoPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        tk.Label(self, text="Starting position:").grid(
            row=0, column=0, padx=10, pady=5, sticky="e"
        )
        self.starting_entry = tk.Entry(self, width=30)
        self.starting_entry.grid(row=0, column=1, padx=10, pady=5)

        tk.Label(self, text="Destination position:").grid(
            row=1, column=0, padx=10, pady=5, sticky="e"
        )
        self.destination_entry = tk.Entry(self, width=30)
        self.destination_entry.grid(row=1, column=1, padx=10, pady=5)

        self.go_and_photo_btn = tk.Button(
            self, text="Go and take a photo", command=self.on_submit
        )
        self.go_and_photo_btn.grid(row=2, column=0, padx=10, pady=10)

        self.cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)
        self.cancel_btn.grid(row=2, column=1, padx=10, pady=10)

        self.result_label = tk.Label(self, text="", fg="blue", font=("Arial", 12))
        self.result_label.grid(row=3, column=0, columnspan=2, pady=10)

    def on_submit(self):
        try:
            start_text = self.starting_entry.get()
            dest_text = self.destination_entry.get()
            route_dict = {"start_name": start_text, "destination_name": dest_text}
            mission_request_dict = {"action": "TakePhoto", "route": route_dict}
            mission_request = {"MissionRequest": mission_request_dict}
            send_general_request(s, mission_request, 1048576)
            time.sleep(1)

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.starting_entry.delete(0, tk.END)
        self.destination_entry.delete(0, tk.END)

    def on_cancel(self):
        self.starting_entry.delete(0, tk.END)
        self.destination_entry.delete(0, tk.END)


class TakePhotoPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)
        self.photo_btn = tk.Button(
            self, text="Take a photo", command=self.on_take_photo
        )
        self.photo_btn.grid(row=2, column=0, padx=10, pady=10)

    def on_take_photo(self):
        try:
            send_photo_request_and_save_photo(s)
        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")


class GoToPositionPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        self.x_label = tk.Label(self, text="x coordinate:", font=("Arial", 12))
        self.y_label = tk.Label(self, text="y coordinate:", font=("Arial", 12))
        self.theta_label = tk.Label(self, text="theta:", font=("Arial", 12))

        self.x_entry = tk.Entry(self, width=30)
        self.y_entry = tk.Entry(self, width=30)
        self.theta_entry = tk.Entry(self, width=30)

        self.x_label.pack(padx=10, pady=(10, 0))
        self.x_entry.pack(padx=10, pady=5)
        self.y_label.pack(padx=10, pady=(10, 0))
        self.y_entry.pack(padx=10, pady=5)
        self.theta_label.pack(padx=10, pady=(10, 0))
        self.theta_entry.pack(padx=10, pady=5)

        go_to_position_btn = tk.Button(
            self, text="  Go  ", command=self.on_go_to_position
        )
        cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        go_to_position_btn.pack(side="left", padx=20, pady=10)
        cancel_btn.pack(side="left", padx=20, pady=10)

    def on_go_to_position(self):
        try:
            x_text = int(self.x_entry.get())
            y_text = int(self.y_entry.get())
            theta_text = int(self.theta_entry.get())
            coordinates_dict = {
                "x_coordinate": x_text,
                "y_coordinate": y_text,
                "theta": theta_text,
            }
            go_to_position_dict = {
                "action": "GoToPosition",
                "route": coordinates_dict,
            }
            go_to_position_request = {"MissionRequest": go_to_position_dict}
            send_general_request(s, go_to_position_request, 1024)
            time.sleep(1)

        except ValueError:
            messagebox.showerror("Error", "The coordinates should be integers!")

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

    def on_cancel(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)


class InsertRackPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        self.x_label = tk.Label(self, text="x coordinate:", font=("Arial", 12))
        self.y_label = tk.Label(self, text="y coordinate:", font=("Arial", 12))
        self.theta_label = tk.Label(self, text="theta:", font=("Arial", 12))
        self.lane_number_label = tk.Label(self, text="lane number:", font=("Arial", 12))

        self.x_entry = tk.Entry(self, width=30)
        self.y_entry = tk.Entry(self, width=30)
        self.theta_entry = tk.Entry(self, width=30)
        self.lane_number_entry = tk.Entry(self, width=30)

        self.x_label.pack(padx=10, pady=(10, 0))
        self.x_entry.pack(padx=10, pady=5)
        self.y_label.pack(padx=10, pady=(10, 0))
        self.y_entry.pack(padx=10, pady=5)
        self.theta_label.pack(padx=10, pady=(10, 0))
        self.theta_entry.pack(padx=10, pady=5)
        self.lane_number_label.pack(padx=10, pady=(10, 0))
        self.lane_number_entry.pack(padx=10, pady=5)

        insert_rack_btn = tk.Button(
            self, text="Insert Rack", command=self.on_insert_rack
        )
        cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        insert_rack_btn.pack(side="left", padx=20, pady=10)
        cancel_btn.pack(side="left", padx=20, pady=10)

    def on_insert_rack(self):
        try:
            x_text = int(self.x_entry.get())
            y_text = int(self.y_entry.get())
            theta_text = int(self.theta_entry.get())
            lane_number_text = int(self.lane_number_entry.get())

            coordinates_dict = {
                "x_coordinate": x_text,
                "y_coordinate": y_text,
                "theta": theta_text,
            }
            insert_rack_dict = {
                "action": "InsertRack",
                "position": coordinates_dict,
                "lane_number": lane_number_text,
            }
            insert_rack_request = {"MissionRequest": insert_rack_dict}
            send_general_request(s, insert_rack_request, 1024)
            time.sleep(1)

        except ValueError:
            messagebox.showerror("Error", "The coordinates should be integers!")

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

    def on_cancel(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)


class RemoveRackPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        self.x_label = tk.Label(self, text="x coordinate:", font=("Arial", 12))
        self.y_label = tk.Label(self, text="y coordinate:", font=("Arial", 12))
        self.theta_label = tk.Label(self, text="theta:", font=("Arial", 12))
        self.lane_number_label = tk.Label(self, text="lane number:", font=("Arial", 12))

        self.x_entry = tk.Entry(self, width=30)
        self.y_entry = tk.Entry(self, width=30)
        self.theta_entry = tk.Entry(self, width=30)
        self.lane_number_entry = tk.Entry(self, width=30)

        self.x_label.pack(padx=10, pady=(10, 0))
        self.x_entry.pack(padx=10, pady=5)
        self.y_label.pack(padx=10, pady=(10, 0))
        self.y_entry.pack(padx=10, pady=5)
        self.theta_label.pack(padx=10, pady=(10, 0))
        self.theta_entry.pack(padx=10, pady=5)
        self.lane_number_label.pack(padx=10, pady=(10, 0))
        self.lane_number_entry.pack(padx=10, pady=5)

        remove_rack_btn = tk.Button(
            self, text="Remove Rack", command=self.on_remove_rack
        )
        cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        remove_rack_btn.pack(side="left", padx=20, pady=10)
        cancel_btn.pack(side="left", padx=20, pady=10)

    def on_remove_rack(self):
        try:
            x_text = int(self.x_entry.get())
            y_text = int(self.y_entry.get())
            theta_text = int(self.theta_entry.get())
            lane_number_text = int(self.lane_number_entry.get())

            coordinates_dict = {
                "x_coordinate": x_text,
                "y_coordinate": y_text,
                "theta": theta_text,
            }
            remove_rack_dict = {
                "action": "RemoveRack",
                "position": coordinates_dict,
                "lane_number": lane_number_text,
            }
            remove_rack_request = {"MissionRequest": remove_rack_dict}
            send_general_request(s, remove_rack_request, 1024)
            time.sleep(1)

        except ValueError:
            messagebox.showerror("Error", "The coordinates should be integers!")

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")

        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

    def on_cancel(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)


class BeerMePage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        self.beer_me_btn = tk.Button(self, text="Beer me", command=self.on_beer_me)
        self.beer_me_btn.grid(row=2, column=0, padx=10, pady=10)

    def on_beer_me(self):
        try:
            beer_me_request = {"BeerMe": None}
            send_general_request(s, beer_me_request, 1024)
            time.sleep(1)

        except Exception as e:
            messagebox.showerror("Error", f"Error: {e}")
