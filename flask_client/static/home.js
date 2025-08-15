document.addEventListener('DOMContentLoaded', (event) => {


  const socket = io();

  const messagesDiv = document.getElementById('messages');
  const chatForm = document.getElementById('chat-form');
  const messageInput = document.getElementById('message-input');


  const addMessage = (content, type) => {
    const msgElement = document.createElement('div');
    msgElement.textContent = content;
    msgElement.classList.add('message', type);
    messagesDiv.appendChild(msgElement);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;
  };


  chatForm.addEventListener('submit', (e) => {
    e.preventDefault();
    const message = messageInput.value.trim();
    if (message) {
      addMessage(message, 'user-message');
      socket.emit('message', message);
      messageInput.value = '';
    }
  });

  socket.on('response', (msg) => {
    addMessage(msg, 'server-response');
    console.log("venit raspunsu ", msg);
  });

  socket.on('connect', () => {
    console.log('Connected to server!');
  });

  socket.on('disconnect', () => {
    console.log('Disconnected from server.');
  });
});

