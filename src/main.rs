use actix_web::{web, App, HttpResponse, HttpServer};
use db::connect_and_setup;
use mongodb::{error::Result, Collection};
use schema::User;

mod db;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
    let collection = connect_and_setup().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .route("/users/{username}", web::get().to(get_user_data))
            .route("/users/create", web::post().to(create_new_user))
    })
    .bind("localhost:9090")?
    .run()
    .await?;

    Ok(())
}

// Handler to get user data
async fn get_user_data(
    path: web::Path<String>, // Directly use the path without destructuring
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    let username = path.into_inner(); // Extract the username from the path
    let user = User {
        username,
        email: String::new(),
        password: String::new(),
    };

    let userdata = user.get_user_data(&collection).await;
    println!("User data: {:?}", userdata);

    match userdata {
        Ok(user_data) => HttpResponse::Ok().json(user_data),
        Err(e) => {
            eprintln!("Error retrieving user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn create_new_user(
    user: web::Json<User>,
    collection: web::Data<Collection<User>>,
) -> HttpResponse {
    println!("Inserting...");
    let result = user.insert_new_user(&collection).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error retrieving user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
