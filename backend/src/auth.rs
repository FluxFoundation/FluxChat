use std::sync::Mutex;

use rocket::{
    Request, State, catch,
    http::{CookieJar, Status},
    outcome::Outcome,
    post,
    request::FromRequest,
    response::Redirect,
    serde::json::Json,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::db::{self, STATE};

pub struct Session {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, (Status, ()), Status> {
        if let Some(cookie) = request.cookies().get_private("user") {
            Outcome::Success(Session {
                username: cookie.value().to_string(),
            })
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}

#[post("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Json<ApiResponse> {
    jar.remove_private("user");
    Json(ApiResponse { message: "logged out".to_string(), redirectUrl: Some("/login".to_string()) })
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
    redirectUrl: Option<String>,
}

#[post("/signup", format = "application/json", data = "<user>")]
pub async fn signup(
    jar: &CookieJar<'_>,
    user: Json<User>,
    state: &State<Mutex<Connection>>,
) -> Json<ApiResponse> {
    let conn = state.lock().unwrap();
    let res = db::add_safe(&conn, &user.username, &user.password).unwrap();

    match res {
        STATE::OK => {
            jar.add_private(("user", user.username.clone()));
            Json(ApiResponse {
                message: "signup successfull".to_string(),
                redirectUrl: Some("/".to_string()),
            })
        }
        STATE::USERALREADYEXIST => Json(ApiResponse {
            message: res.to_string(),
            redirectUrl: None,
        }),
        _ => unreachable!(),
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    jar: &CookieJar<'_>,
    user: Json<User>,
    state: &State<Mutex<Connection>>,
) -> Json<ApiResponse> {
    let conn = state.lock().unwrap();
    let res = db::verify_safe(&conn, &user.username, &user.password).unwrap();

    match res {
        STATE::OK => {
            jar.add_private(("user", user.username.clone()));
            Json(ApiResponse {
                message: "login successfull".to_string(),
                redirectUrl: Some("/".to_string()),
            })
        }
        STATE::USERDOESNOTEXIST | STATE::PASSWORDMISMATCH => Json(ApiResponse {
            message: res.to_string(),
            redirectUrl: None,
        }),
        _ => unreachable!(),
    }
}
