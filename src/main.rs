use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;
mod connections;
mod types;
use connections::connect;
use types::{ChatRooms, Clients, Room};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = init_ws().await;

    // clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // rooms things
    let chat_rooms: ChatRooms = Arc::new(Mutex::new(HashMap::new()));
    let room_id = Uuid::new_v4();
    let new_room: Room = Room {
        room_id,
        users: Vec::new(),
    };
    chat_rooms.lock().await.insert(room_id, new_room);

    while let Ok((stream, _)) = listener.accept().await {
        let clients_clone = Arc::clone(&clients);
        tokio::spawn(connect(stream, clients_clone));
    }
}

pub async fn init_ws() -> TcpListener {
    let addr: SocketAddr = "192.168.0.127:8080"
        .to_string()
        .parse()
        .expect("Invalid Address");
    println!("WS server online at: {}", addr);

    let listener: TcpListener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address.");

    listener
}
