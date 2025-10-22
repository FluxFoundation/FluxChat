use rocket::{fs::FileServer, get, launch, routes};
use rocket::fs::NamedFile;
use std::path::Path;

#[get("/login")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/index.html")).await.ok()
}

#[get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("../FrontEnd/dist/home.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, home])
        .mount("/assets", FileServer::from("../FrontEnd/dist/assets"))
}
