use crate::schema::User;
use dotenv::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};
use std::env;
use tokio;

#[tokio::main]
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

    let new_user = User {
        username: "chirag".to_string(),
        email: "jani@test.com".to_string(),
        password: "janijani".to_string(),
    };

    let new_user1 = User {
        username: "chirag1".to_string(),
        email: "jani@test.com".to_string(),
        password: "janijani".to_string(),
    };
    let new_user2 = User {
        username: "chirag2".to_string(),
        email: "jani@test.com".to_string(),
        password: "janijani".to_string(),
    };
    let new_user3 = User {
        username: "chirag3".to_string(),
        email: "jani@test.com".to_string(),
        password: "janijani".to_string(),
    };

    let mut userlist = Vec::<User>::new();

    userlist.push(new_user1);
    userlist.push(new_user2);
    userlist.push(new_user3);

    let _ = new_user.clone().insert_new_user(&collection).await;
    let _ = new_user
        .clone()
        .insert_multiple(&collection, userlist)
        .await;
    let _ = new_user
        .get_user_data(&collection, "chirag3".to_string(), "janijani".to_string())
        .await;
    Ok(collection)
}

// * creating collection thing
pub fn create_new_collection(database: Database, name: &str) -> Result<Collection<User>, Error> {
    Ok(database.collection(name))
}
