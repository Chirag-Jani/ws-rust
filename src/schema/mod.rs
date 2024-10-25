use mongodb::{bson::doc, error::Error, Collection};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
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

    pub async fn insert_multiple(
        &self,
        collection: &Collection<Self>,
        users: Vec<Self>,
    ) -> Result<(), Error> {
        let _result = collection.insert_many(users).await;
        Ok(())
    }

    // * find user with username and password
    pub async fn get_user_data(
        &self,
        collection: &Collection<Self>,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let mut cursor = collection
            .find(doc! {"username": username, "password": password})
            .await?;

        while cursor.advance().await.unwrap() {
            println!("User: {:?}", cursor.current().get("username").unwrap());
            println!("User: {:?}", cursor.current().get("email").unwrap());
            println!("User: {:?}", cursor.current().get("password").unwrap());
        }

        Ok(())
    }
}
