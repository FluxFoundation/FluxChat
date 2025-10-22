
pub mod auth;
pub mod db;
pub mod pw;


use rocket::catchers;
use rocket::{fs::FileServer, get, launch, routes};
use rocket::fs::NamedFile;
use std::path::Path;
use std::sync::Mutex;



#[get("/login")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/index.html")).await.ok()
}

#[get("/")]
async fn home(s: auth::Session) -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/home.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    let conn = db::initialise_db().expect("database init");
    rocket::build()
        .manage(Mutex::new(conn))
        .mount("/", routes![
            index, 
            home
        ])
        .mount("/api/v1", routes![

        ])
        .mount("/assets", FileServer::from("../FrontEnd/dist/assets"))
        .register("/", catchers![
            auth::unauthorized
        ])
}
