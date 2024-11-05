use futures::{SinkExt, StreamExt};
use log::error;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc::UnboundedSender, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

type Clients = Arc<Mutex<HashMap<usize, UnboundedSender<Message>>>>;
static mut CLIENT_ID: usize = 0;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = "192.168.0.127:8080"
        .to_string()
        .parse()
        .expect("Invalid Address");
    println!("WS server online at: {}", addr);

    let listener: TcpListener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address.");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients_clone = Arc::clone(&clients);
        tokio::spawn(handle_connection(stream, clients_clone));
    }
}

async fn handle_connection(stream: TcpStream, clients: Clients) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    let client_id;
    unsafe {
        CLIENT_ID += 1;
        client_id = CLIENT_ID;
    }

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id, tx);
    }

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
                    // if id != client_id {
                    let response = format!("Client {}: {}", client_id, text);
                    if tx.send(Message::Text(response)).is_err() {
                        error!("Error sending message to client {}", id);
                    }
                    // }
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
