use std::sync::{Arc, Mutex, RwLock};

use rocket::{delete, get, post, routes, serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

use crate::{
    astek::{indisponibility::Indisponibility, Astek},
    helpers::{request::Request, response::Response, AlreadyExists, InternalError},
    middlewares::{
        astek,
        auth::{AuthenticatedUser, KeyStore},
        get_state_mut,
    },
    state::IntrastekState,
};

use log::info;

pub fn load_asteks(rocket: Rocket<Build>) -> Rocket<Build> {
    info!("Loading asteks...");
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

// #[post("/", data = "<req>")]
// async fn register_asteks(
//     req: Json<Request<Uuid>>,
//     state: &State<Mutex<IntrastekState>>,
// ) -> Response<&'static str, String> {
//     get_state_mut(state, |mutex| {
//         if mutex
//             .asteks
//             .iter()
//             .any(|a| a.as_ref().read().is_ok_and(|x| x.id == req.data))
//         {
//             return Err(Box::new(AlreadyExists { data: req.data }));
//         }
//         mutex
//             .asteks
//             .push(Arc::new(RwLock::new(Astek::new(req.data))));
//         Ok("Ok")
//     })
//     .into()
// }

#[post("/", data = "<req>")]
async fn register_asteks(
    _user: AuthenticatedUser,
    req: Json<Request<Uuid>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<&'static str, String> {
    get_state_mut(state, |mutex| {
        if mutex
            .asteks
            .iter()
            .any(|a| a.as_ref().read().is_ok_and(|x| x.id == req.data))
        {
            return Err(Box::new(AlreadyExists { data: req.data }));
        }
        mutex
            .asteks
            .push(Arc::new(RwLock::new(Astek::new(req.data))));
        Ok("Ok")
    })
    .into()
}

#[get("/")]
async fn get_asteks(state: &State<Mutex<IntrastekState>>) -> Response<Vec<Astek>, String> {
    get_state_mut(state, |mutex| {
        Ok(mutex
            .asteks
            .iter()
            .flat_map(|a| match a.as_ref().read() {
                Ok(astek) => Ok(astek.clone()),
                Err(_) => Err(Box::new(InternalError)),
            })
            .collect())
    })
    .into()
}

#[get("/<id>")]
async fn get_astek(id: Uuid, state: &State<Mutex<IntrastekState>>) -> Response<Astek, String> {
    astek::get_astek(id, state).into()
}

#[post("/<id>", data = "<req>")]
async fn add_indisponibility(
    id: Uuid,
    req: Json<Request<Indisponibility>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<usize, String> {
    astek::get_astek_and_then(id, state, |astek| match astek.as_ref().write() {
        Ok(mut astk) => Ok(astk.add_indisponibility(req.data.clone())),
        Err(_) => Err(Box::new(InternalError)),
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
