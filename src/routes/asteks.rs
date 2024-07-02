use std::sync::{Arc, Mutex, RwLock};

use rocket::{get, post, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    astek::Astek,
    helpers::{request::Request, response::Response},
    state::IntrastekState,
};

#[post("/asteks", data = "<informations>")]
pub async fn register_asteks(
    informations: Json<Request<Uuid>>,
    state: &State<Mutex<IntrastekState>>,
) -> Json<Response<()>> {
    let astek = Arc::new(RwLock::new(Astek::new(informations.0.data).unwrap()));

    state.lock().unwrap().asteks.push(astek.clone());
    Json(Response { data: () })
}

#[get("/asteks")]
pub async fn get_asteks(state: &State<Mutex<IntrastekState>>) -> Json<Response<Vec<Astek>>> {
    let mut asteks: Vec<Astek> = Vec::new();

    state.lock().unwrap().asteks.iter().for_each(|astk| {
        if let Ok(astek) = astk.as_ref().read() {
            asteks.push(astek.clone());
        }
    });

    Json(Response { data: asteks })
}
