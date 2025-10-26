#![allow(non_snake_case)]

pub mod auth;
pub mod db;
pub mod pw;

use base64::Engine;
use base64::engine::general_purpose;
use dotenv::dotenv;
use rocket::config::SecretKey;
use rocket::fs::NamedFile;
use rocket::{Config, catchers};
use rocket::{fs::FileServer, get, launch, routes};
use std::env;
use std::path::Path;
use std::sync::Mutex;

#[get("/login")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/index.html"))
        .await
        .ok()
}

#[get("/")]
async fn home(s: auth::Session) -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/home.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let secret_key_str =
        env::var("ROCKET_SECRET_KEY").expect("ROCKET_SECRET_KEY must be set in .env");

    let secret_key = SecretKey::from(
        &general_purpose::STANDARD
            .decode(secret_key_str)
            .expect("Invalid base64 key"),
    );

    let config = Config {
        secret_key,
        ..Config::default()
    };

    let conn = db::initialise_db().expect("database init");
    rocket::custom(config)
        .manage(Mutex::new(conn))
        .mount("/", routes![index, home])
        .mount("/api/v1", routes![auth::signup, auth::login, auth::logout])
        .mount("/assets", FileServer::from("../FrontEnd/dist/assets"))
        .register("/", catchers![auth::unauthorized])
}
