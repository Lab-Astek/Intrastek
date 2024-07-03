use std::sync::{Arc, Mutex, RwLock};

use rocket::{delete, get, post, routes, serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

use crate::{
    astek::{indisponibility::Indisponibility, Astek},
    helpers::{request::Request, response::Response, IntrastekErrors},
    middlewares,
    state::IntrastekState,
};

pub fn load_asteks(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/asteks",
        routes![
            register_asteks,
            get_asteks,
            get_astek,
            add_indisponibility,
            delete_astek,
            delete_indisponibility
        ],
    )
}

#[post("/", data = "<req>")]
async fn register_asteks(
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

#[get("/")]
async fn get_asteks(state: &State<Mutex<IntrastekState>>) -> Response<Vec<Astek>, String> {
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

#[get("/<id>")]
async fn get_astek(id: Uuid, state: &State<Mutex<IntrastekState>>) -> Response<Astek, String> {
    middlewares::astek::get_astek(id, state).into()
}

#[post("/<id>", data = "<req>")]
async fn add_indisponibility(
    id: Uuid,
    req: Json<Request<Indisponibility>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<usize, String> {
    middlewares::astek::get_astek_and_then(id, state, |astek| {
        astek
            .as_ref()
            .write()
            .map(|mut a| a.add_indisponibility(req.data.clone()))
            .map_err(|_| IntrastekErrors::InternalError)
    })
    .into()
}

#[delete("/<id>")]
async fn delete_astek(
    id: Uuid,
    state: &State<Mutex<IntrastekState>>,
) -> Response<&'static str, String> {
    match state.lock() {
        Ok(mut mutex) => {
            if let Some(pos) = mutex
                .asteks
                .iter()
                .position(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                mutex.asteks.remove(pos);
                Response::ok(200, "Ok")
            } else {
                Response::err(404, String::from("Ressource not found"))
            }
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[delete("/<id>/indisponibilities/<indisponibility_id>")]
async fn delete_indisponibility(
    id: Uuid,
    indisponibility_id: usize,
    state: &State<Mutex<IntrastekState>>,
) -> Response<&'static str, &'static str> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(astek) = mutex
                .asteks
                .iter()
                .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                if let Ok(mut astek) = astek.as_ref().write() {
                    astek.remove_indisponibility(indisponibility_id);
                    Response::ok(200, "Ok")
                } else {
                    Response::err(500, "Internal error")
                }
            } else {
                Response::err(404, "Ressource not found")
            }
        }
        Err(_) => Response::err(500, "Internal error"),
    }
}
