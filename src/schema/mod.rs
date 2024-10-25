use mongodb::{bson::doc, error::Error, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    // * inserting user thing
    pub async fn insert_new_user(&self, collection: &Collection<Self>) -> Result<(), Error> {
        let result = collection.insert_one(self).await;
        println!("Inserted User: {:?}", result.unwrap());
        Ok(())
    }

    // * find user with username and password
    pub async fn get_user_data(&self, collection: &Collection<Self>) -> Result<(), Error> {
        let mut cursor = collection
            .find(doc! {"username": self.username.clone()})
            .await?;

        let resp = cursor.current();
        
        println!("Repsonse: {:?}", resp);

        while cursor.advance().await.unwrap() {
            println!("Username: {:?}", cursor.current().get("username").unwrap());
            println!("Email: {:?}", cursor.current().get("email").unwrap());
            println!("Password: {:?}", cursor.current().get("password").unwrap());
        }

        Ok(())
    }
}
