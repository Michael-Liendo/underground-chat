use socketioxide::extract::SocketRef;

pub mod join;
pub mod message;

pub fn on_connect(socket: SocketRef) {
    socket.on("join", join::on_join);

    socket.on("chat message", message::on_chat_message);
}
