use rocket::{catch, http::Status, outcome::Outcome, request::FromRequest, response::Redirect, Request};


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

pub struct User {
    pub username: String,
    pub password: String,
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}