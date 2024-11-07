use futures::{SinkExt, StreamExt};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use uuid::Uuid;

use crate::types::Clients;

pub async fn connect(stream: TcpStream, clients: Clients) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    let client_id = Uuid::new_v4();
    println!("Generated UUID is: {}", client_id);

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id, tx);
    }
    sender
        .send(Message::Text(format!(
            "Connected Successfully with ID: {}",
            client_id.to_string(),
        )))
        .await
        .unwrap();

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                error!("Failed to send message to client {}", client_id);
                break;
            }
        }
    });

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received message from client {}: {}", client_id, text);

                let clients_lock = clients.lock().await;
                for (&id, tx) in clients_lock.iter() {
                    if id != client_id {
                        let response = format!("Client {}: {}", client_id, text);
                        if tx.send(Message::Text(response)).is_err() {
                            error!("Error sending message to client {}", id);
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

    clients.lock().await.remove(&client_id);
    println!("Client {} removed.", client_id);
}
