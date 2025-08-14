        document.addEventListener('DOMContentLoaded', (event) => {
            // Establish a connection to the server.
            // By default, it connects to the host that serves the page.
            const socket = io();

            const messagesDiv = document.getElementById('messages');
            const chatForm = document.getElementById('chat-form');
            const messageInput = document.getElementById('message-input');

            // Function to add a message to the chat window
            const addMessage = (content, type) => {
                const msgElement = document.createElement('div');
                msgElement.textContent = content;
                msgElement.classList.add('message', type);
                messagesDiv.appendChild(msgElement);
                // Scroll to the bottom of the message window
                messagesDiv.scrollTop = messagesDiv.scrollHeight;
            };

            // Handle form submission
            chatForm.addEventListener('submit', (e) => {
                e.preventDefault(); // Prevent page reload
                const message = messageInput.value.trim();
                if (message) {
                    // Display the user's message
                    addMessage(message, 'user-message');
                    // Send the message to the server via the 'message' event
                    socket.emit('message', message);
                    // Clear the input field
                    messageInput.value = '';
                }
            });

            // Listen for the 'response' event from the server
            socket.on('response', (msg) => {
                // Display the server's response
                addMessage(msg, 'server-response');
            });

            // Optional: Handle connection events for debugging
            socket.on('connect', () => {
                console.log('Connected to server!');
            });

            socket.on('disconnect', () => {
                console.log('Disconnected from server.');
            });
        });
    
