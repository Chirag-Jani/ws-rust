use futures::{SinkExt, StreamExt};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use uuid::Uuid;

use crate::types::{ChatRooms, Clients, Room};

pub async fn connect(stream: TcpStream, clients: Clients, chat_rooms: ChatRooms) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    let client_id = Uuid::new_v4();
    println!("Generated UUID for client: {}", client_id);

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Add the client to the global clients map
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id, tx);
    }

    sender
        .send(Message::Text(format!(
            "Connected successfully with ID: {}. Please send the room ID to join.",
            client_id
        )))
        .await
        .unwrap();

    // Wait for the client to send the room ID
    let room_id = if let Some(Ok(Message::Text(room_id_text))) = receiver.next().await {
        match Uuid::parse_str(&room_id_text) {
            Ok(id) => id,
            Err(_) => {
                sender
                    .send(Message::Text("Invalid room ID format.".to_string()))
                    .await
                    .unwrap();
                return;
            }
        }
    } else {
        return;
    };

    // Add the client to the specified room
    {
        let mut chat_rooms_lock = chat_rooms.lock().await;
        let room = chat_rooms_lock.entry(room_id).or_insert_with(|| Room {
            room_id,
            users: Vec::new(),
        });
        room.users.push(client_id);
    }

    // Send confirmation of room join
    sender
        .send(Message::Text(format!("Joined room with ID: {}", room_id)))
        .await
        .unwrap();

    // Spawn a task to send messages from `rx` to this client
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                error!("Failed to send message to client {}", client_id);
                break;
            }
        }
    });

    // Handle incoming messages and broadcast within the room
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received message from client {}: {}", client_id, text);

                let chat_rooms_lock = chat_rooms.lock().await;
                if let Some(room) = chat_rooms_lock.get(&room_id) {
                    for &user_id in &room.users {
                        if user_id != client_id {
                            if let Some(tx) = clients.lock().await.get(&user_id) {
                                let response = format!("Client {}: {}", client_id, text);
                                if tx.send(Message::Text(response)).is_err() {
                                    error!("Error sending message to client {}", user_id);
                                }
                            }
                        }
                    }
                }
            }

            Ok(Message::Close(_)) => {
                println!("Client {} disconnected.", client_id);
                break;
            }

            Ok(_) => (),

            Err(e) => {
                error!("Error processing message for client {}: {}", client_id, e);
                break;
            }
        }
    }

    // Cleanup: remove client from the room and global clients map
    {
        let mut chat_rooms_lock = chat_rooms.lock().await;
        if let Some(room) = chat_rooms_lock.get_mut(&room_id) {
            room.users.retain(|&id| id != client_id);
        }
        clients.lock().await.remove(&client_id);
    }
    println!("Client {} removed.", client_id);
}
