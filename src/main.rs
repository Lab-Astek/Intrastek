#![allow(dead_code)]

use env_logger::{Builder, Env};
use rocket::routes;
use routes::{
    asteks::{get_asteks, register_asteks},
    ping,
};
use state::IntrastekState;
use std::sync::Mutex;

mod activity;
mod astek;
mod helpers;
mod interval;
mod module;
mod planner;
mod routes;
mod state;

#[rocket::main]
async fn main() -> Result<(), String> {
    let env = Env::new().filter("ASSIGN_LOG");
    Builder::from_env(env).init();

    rocket::build()
        .mount("/", routes![register_asteks, ping, get_asteks])
        .manage(Mutex::new(IntrastekState::default()))
        .launch()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
