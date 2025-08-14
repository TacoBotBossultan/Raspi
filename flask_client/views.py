from flask import  render_template
# app.py
from flask_client import app
from flask_socketio import SocketIO, emit

socketio = SocketIO(app)

@app.route('/')
def index():
    """
    Serves the main HTML page for the chat application.
    """
    return render_template('home.html')

@socketio.on('message')
def handle_message(msg):
    """
    Handles a new message received from a client.
    It prints the received message to the console and sends back a static response.
    """
    print('Message from client: ' + msg)
    # The 'emit' function sends an event to the client.
    # We're defining a custom event named 'response'.
    emit('response', 'lmao')

if __name__ == '__main__':
    # The 'run' method starts the development server.
    # We use socketio.run to ensure the WebSocket server is started correctly.
    # Using eventlet is a common choice for production-ready WebSocket servers with Flask.
    socketio.run(app, debug=True)

