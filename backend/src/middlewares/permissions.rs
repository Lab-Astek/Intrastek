use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::{
    db::{user, Permissions},
    state::IntrastekState,
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Permissions {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mail = match request.headers().get_one("X-email") {
            Some(h) => h,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let perm: Option<Permissions> =
            if let Some(state) = request.rocket().state::<IntrastekState>() {
                let res = state
                    .db
                    .user()
                    .find_unique(user::email::equals(mail.to_string()))
                    .exec()
                    .await;
                match res {
                    Ok(Some(d)) => Some(d.permissions),
                    _ => None,
                }
            } else {
                None
            };

        match perm {
            Some(p @ (Self::Ape | Self::Astek)) => Outcome::Success(p),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
