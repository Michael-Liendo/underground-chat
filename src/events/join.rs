use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};

#[derive(Debug, Deserialize, Serialize)]
pub struct Join {
    room: String,
    username: String,
}

pub fn on_join(socket: SocketRef, Data(join): Data<Join>) {
    socket.leave_all().ok();
    socket.join(join.room.clone()).ok();

    socket
        .within(join.room.clone())
        .broadcast()
        .emit("join", join)
        .ok();
}
