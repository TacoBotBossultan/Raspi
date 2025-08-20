import tkinter as tk
from tkinter import ttk  

class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("TacoBot user interface")
        self.geometry("1000x700")

        self.container = tk.Frame(self)
        self.container.pack(fill="both", expand=True)

        self.pages = {}

        for P in (DefineHomePage, StoreRoutePage, TakePhotoPage):
            page_name = P.__name__
            frame = P(parent=self.container, controller=self)
            self.pages[page_name] = frame
            frame.grid(row=0, column=0, sticky="nsew")

        nav_frame = tk.Frame(self)
        nav_frame.pack(side="top", fill="x")

        define_home_button = tk.Button(nav_frame, text="Define home", command=lambda: self.show_page("DefineHomePage"))
        store_route_button = tk.Button(nav_frame, text="Store route", command=lambda: self.show_page("StoreRoutePage"))
        take_photo_button = tk.Button(nav_frame, text="Take photo", command=lambda: self.show_page("TakePhotoPage"))

        define_home_button.pack(side="left", expand=True, fill="x")
        store_route_button.pack(side="left", expand=True, fill="x")
        take_photo_button.pack(side="left", expand=True, fill="x")

        self.show_page("DefineHomePage")  

    def show_page(self, page_name):
        page = self.pages[page_name]
        page.tkraise()


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
        self.theta_entry.pack(padx= 10, pady=5)

        self.result_label = tk.Label(self, text="", font=("Arial", 12))
        self.result_label.pack(pady=10)

        define_home_btn = tk.Button(self, text="Submit", command=self.on_define_home)
        cancel_btn = tk.Button(self, text="Cancel", command=self.on_cancel)

        define_home_btn.pack(side="left", padx=20, pady=10)
        cancel_btn.pack(side="left", padx=20, pady=10)

    def on_define_home(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

        self.result_label.config(text="✅ You defined the home")

    def on_cancel(self):
        self.x_entry.delete(0, tk.END)
        self.y_entry.delete(0, tk.END)
        self.theta_entry.delete(0, tk.END)

        self.result_label.config(text="")
    

class StoreRoutePage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)

        tk.Label(self, text="Starting position:").grid(row=0, column=0, padx=10, pady=5, sticky="e")
        self.starting_position_field = tk.Entry(self, width=30)
        self.starting_position_field.grid(row=0, column=1, padx=10, pady=5)

        tk.Label(self, text="Destination position:").grid(row=1, column=0, padx=10, pady=5, sticky="e")
        self.destination_position_field = tk.Entry(self, width=30)
        self.destination_position_field.grid(row=1, column=1, padx=10, pady=5)

        self.define_route_btn = tk.Button(self, text="Define route", command=self.show_more_fields)
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

            self.direction_entry = ttk.Combobox(self, values=[
                "Forward", "Backward", "Right",
                "Left", "Rotate Right", "Rotate Left"
            ])
            self.direction_entry.current(0)  
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
        if self.direction_label: self.direction_label.grid_forget()
        if self.direction_entry: self.direction_entry.grid_forget()
        if self.value_label: self.value_label.grid_forget()
        if self.value_entry: self.value_entry.grid_forget()
        if self.submit_btn: self.submit_btn.grid_forget()
        if self.stop_btn: self.stop_btn.grid_forget()

        first_text = self.starting_position_field.get()
        second_text = self.destination_position_field.get()
        summary = f"Summary:\n\nStarting position: {first_text}\nDestination position: {second_text}\n\nRoute:\n"
        for i, (opt, txt) in enumerate(self.submitted_texts, start=1):
            summary += f"  Route step {i}: {opt} | {txt}\n"

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
        self.on_cancel()

class TakePhotoPage(tk.Frame):
    def __init__(self, parent, controller):
        super().__init__(parent)
        label = tk.Label(self, text="Here is take photo page", font=("Arial", 18))
        label.pack(pady=20)


if __name__ == "__main__":
    app = App()
    app.mainloop()

