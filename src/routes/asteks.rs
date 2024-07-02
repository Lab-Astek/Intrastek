use std::sync::{Arc, Mutex, RwLock};

use rocket::{get, post, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    astek::Astek,
    helpers::{request::Request, response::Response},
    state::IntrastekState,
};

#[post("/asteks", data = "<req>")]
pub async fn register_asteks(
    req: Json<Request<Uuid>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<&'static str, String> {
    let astek = Arc::new(RwLock::new(Astek::new(req.0.data).unwrap()));

    match state.lock() {
        Ok(mut mutex) => {
            if let Some(_) = mutex
                .asteks
                .iter()
                .position(|a| a.as_ref().read().is_ok_and(|x| x.id == req.data))
            {
                return Response::err(409, format!("{} already exists.", req.data));
            }
            mutex.asteks.push(astek.clone());
            Response::ok(200, "Ok")
        }
        Err(_) => Response::err(500, String::from("Internal Error")),
    }
}

#[get("/asteks")]
pub async fn get_asteks(
    state: &State<Mutex<IntrastekState>>,
) -> Response<Vec<Astek>, &'static str> {
    let mut asteks: Vec<Astek> = Vec::new();

    state.lock().unwrap().asteks.iter().for_each(|astk| {
        if let Ok(astek) = astk.as_ref().read() {
            asteks.push(astek.clone());
        }
    });

    Response::ok(200, asteks)
}
