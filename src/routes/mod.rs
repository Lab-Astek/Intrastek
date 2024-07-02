use std::sync::Mutex;

use rocket::{get, State};

use crate::state::IntrastekState;

pub mod asteks;
pub mod activities;

#[get("/")]
pub async fn ping(_state: &State<Mutex<IntrastekState>>) -> &'static str {
    "pong"
}
