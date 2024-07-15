#![allow(dead_code)]

use env_logger::{Builder, Env};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::init_router;
use state::IntrastekState;
use std::sync::{Arc, Mutex};

mod activity;
mod astek;
mod helpers;
mod interval;
mod middlewares;
mod module;
mod planner;
mod routes;
mod state;

#[allow(warnings)]
mod db;

use crate::middlewares::auth::KeyStore;

#[rocket::main]
async fn main() -> Result<(), String> {
    let env = Env::new().filter("INTRASTEK_LOG");
    Builder::from_env(env).init();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let key_store = Arc::new(KeyStore::new().await);
    let rocket = rocket::build();
    init_router(rocket)
        .manage(key_store)
        .manage(Mutex::new(IntrastekState::default()))
        .attach(cors.to_cors().unwrap())
        .launch()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
