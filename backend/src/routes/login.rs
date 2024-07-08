use std::sync::Mutex;

use rocket::{get, post, routes, serde::json::Json, Build, Rocket, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    astek::indisponibility::Indisponibility,
    helpers::request::Request,
    // middlewares::{get_state, get_state_mut},
    state::IntrastekState,
};

pub fn load_login(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/login",
        routes![create_astek, login_astek, login, get_login],
    )
}

#[get("/")]
async fn login(_state: &State<Mutex<IntrastekState>>) -> &'static str {
    "login page"
}

#[post("/<_email>", data = "<_req>")]
async fn login_astek(
    _email: String,
    _req: Json<Request<Indisponibility>>,
    _state: &State<Mutex<IntrastekState>>,
) -> &'static str {
    "post login"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String, // change to client id for auth??
}

#[post("/", data = "<_email>")]
async fn create_astek(
    _email: Json<Request<LoginRequest>>,
    _state: &State<Mutex<IntrastekState>>,
) -> &'static str {
    "create astek"
}

#[get("/<_id>")]
async fn get_login(_id: Uuid, _state: &State<Mutex<IntrastekState>>) -> &'static str {
    "get login"
}
