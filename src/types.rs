use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, Mutex};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

pub type Clients = Arc<Mutex<HashMap<Uuid, UnboundedSender<Message>>>>;
pub type ChatRooms = Arc<Mutex<HashMap<Uuid, Room>>>;

pub struct Room {
    pub room_id: Uuid,
    pub users: Vec<UnboundedSender<Message>>,
}
