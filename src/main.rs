use futures::{SinkExt, StreamExt};
use log::{error, info};
// use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub mod server1;

#[tokio::main]
async fn main() {
    // logger thing
    env_logger::init();

    // let addr = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let addr: SocketAddr = "127.0.0.1:8080"
        .to_string()
        .parse()
        .expect("Invalid Address");

    // server thing ig
    let listener: TcpListener = TcpListener::bind(&addr)
        .await
        .expect("Failied while Address binding.");

    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a new task for each connection
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    println!("Sender is: {:?}\nReceiver is: {:?}", sender, receiver);

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Message is: {:?}", text);

                let response = "Hello from Server.".to_string();

                if let Err(e) = sender.send(Message::Text(response)).await {
                    error!("Error sending message: {}", e);
                }
            }

            Ok(Message::Close(_)) => break,

            Ok(_) => (),

            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}
