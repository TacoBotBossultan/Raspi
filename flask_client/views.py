from flask import render_template

import connect_to_server
from . import app
from connect_ghipitty import jesus_take_the_wheel, query_ghiptty
from connect_to_server import connect_to_rust_server, send_request
from flask_socketio import SocketIO, emit

HOST = "127.0.0.1"
PORT = 8080

socketio = SocketIO(app)
soseata_rust = "nimic"
connected = False


@app.route("/")
def index():
    return render_template("home.html")


@socketio.on("message")
def handle_message(msg):
    print("Message from client: " + msg)
    global connected, soseata_rust
    if connected:
        res = query_ghiptty(msg)
        print("Cea zis ghiptty:", res)
        if res != None:
            emit("response", res)
        else:
            emit("response", "N-a dat shitpitty niciun raspuns inapoi scz... zoinks")
    else:
        emit(
            "response",
            "Stai ba ca nici nu e conectat la serveru ala jegos de Rust nuj dc, incerc din nou sa ma contectez ... scrie-mi si tu mai incolo",
        )
        soseata_rust, connected = connect_to_rust_server()


if __name__ == "__main__":
    soseata_rust, connected = connect_to_rust_server()
    socketio.run(app, debug=True)
