use futures::{SinkExt, StreamExt};
use log::error;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

type Clients = Arc<Mutex<HashMap<usize, tokio_tungstenite::WebSocketStream<TcpStream>>>>;
static mut CLIENT_ID: usize = 0;

#[tokio::main]
async fn main() {
    // logger thing
    env_logger::init();

    let addr: SocketAddr = "0.0.0.0:8080".to_string().parse().expect("Invalid Address");
    println!("WS thing online at: {}", addr);

    // server thing ig
    let listener: TcpListener = TcpListener::bind(&addr)
        .await
        .expect("Failied while Address binding.");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        // users clone
        let clients_clone = Arc::clone(&clients);
        // Spawn a new task for each connection
        tokio::spawn(handle_connection(stream, clients_clone));
    }
}

async fn handle_connection(stream: TcpStream, clients: Clients) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    println!("--------------------------- WS STream is: {:?}", ws_stream);

    // Generate a unique client ID
    let client_id;
    unsafe {
        CLIENT_ID += 1;
        client_id = CLIENT_ID;
    }

    {
        let mut client_lock = clients.lock().await;
        // client_lock.insert(client_id, ws_stream);
    }

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    println!("Sender is: {:?}\nReceiver is: {:?}", sender, receiver);

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Message is: {:?}", text);

                if let Err(e) = sender.send(Message::Text(text.clone())).await {
                    error!("Error sending message: {}", e);
                }

                let mut clients_lock = clients.lock().await;
                for (id, client) in clients_lock.iter_mut() {
                    if *id != client_id {
                        let response = format!("Client {}: {}", client_id, text);
                        if let Err(e) = client.send(Message::Text(response)).await {
                            error!("Error sending message: {}", e);
                        }
                    }
                }
            }

            Ok(Message::Close(_)) => {
                println!("Connection Closed.");
                break;
            }

            Ok(_) => (),

            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}
