let socket = new WebSocket("ws://" + document.location.hostname + ":8081");
socket.addEventListener("message", console.log);
socket.addEventListener("open", () => {
  socket.send("HI");
});
