#![allow(dead_code)]

use env_logger::{Builder, Env};
use routes::init_router;
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

    let rocket = rocket::build();
    init_router(rocket)
        .manage(Mutex::new(IntrastekState::default()))
        .launch()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
