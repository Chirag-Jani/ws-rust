use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
mod connections;
mod types;
use connections::connect;
use types::{ChatRooms, Clients};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = init_ws().await;

    // Clients and chat rooms
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let chat_rooms: ChatRooms = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients_clone = Arc::clone(&clients);
        let chat_rooms_clone = Arc::clone(&chat_rooms);
        tokio::spawn(connect(stream, clients_clone, chat_rooms_clone));
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
