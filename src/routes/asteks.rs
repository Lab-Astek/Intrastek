use std::sync::{Arc, Mutex, RwLock};

use rocket::{get, post, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    astek::{indisponibility::Indisponibility, Astek},
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
                return Response::err(409, format!("{} already exists", req.data));
            }
            mutex.asteks.push(astek.clone());
            Response::ok(200, "Ok")
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[get("/asteks")]
pub async fn get_asteks(state: &State<Mutex<IntrastekState>>) -> Response<Vec<Astek>, String> {
    let mut asteks: Vec<Astek> = Vec::new();

    match state.lock() {
        Ok(mutex) => {
            mutex.asteks.iter().for_each(|astek| {
                if let Ok(astek) = astek.as_ref().read() {
                    asteks.push(astek.clone());
                }
            });
            Response::ok(200, asteks)
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[get("/asteks/<id>")]
pub async fn get_astek(id: Uuid, state: &State<Mutex<IntrastekState>>) -> Response<Astek, String> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(astek) = mutex
                .asteks
                .iter()
                .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                if let Ok(astek) = astek.as_ref().read() {
                    Response::ok(200, astek.clone())
                } else {
                    Response::err(500, String::from("Internal error"))
                }
            } else {
                Response::err(404, String::from("Ressource not found"))
            }
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[post("/asteks/<id>", data = "<req>")]
pub async fn add_indisponibility(
    id: Uuid,
    req: Json<Request<Indisponibility>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<&'static str, String> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(astek) = mutex
                .asteks
                .iter()
                .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                if let Ok(mut astek) = astek.as_ref().write() {
                    astek.add_indisponibility(req.data.clone());
                    Response::ok(200, "Ok")
                } else {
                    Response::err(500, String::from("Internal error"))
                }
            } else {
                Response::err(404, String::from("Ressource not found"))
            }
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}
