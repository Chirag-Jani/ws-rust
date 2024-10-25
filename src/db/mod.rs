use crate::schema::User;
use dotenv::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};
use std::env;

pub async fn connect_and_setup() -> Result<Collection<User>, Error> {
    dotenv().ok();

    // * env var thing
    let mongo_uri =
        env::var("MONGO_URI").unwrap_or_else(|_e| "mongodb://localhost:27017".to_string());

    // ? idk wot it is
    let mongo_client_options = ClientOptions::parse(mongo_uri).await?;

    // ? idk wot it is
    let mongo_client = Client::with_options(mongo_client_options)?;

    // * creating db maybe
    let database = mongo_client.database("ChatThingDB");
    let collection: Collection<User> = create_new_collection(database, "users").unwrap();

    Ok(collection)
}

// * creating collection thing
pub fn create_new_collection(database: Database, name: &str) -> Result<Collection<User>, Error> {
    Ok(database.collection(name))
}
