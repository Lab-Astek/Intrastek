use rocket::{delete, get, post, routes, serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

use crate::{
    astek::{indisponibility::Indisponibility, Astek},
    helpers::{request::Request, response::Response, AlreadyExists, InternalError},
    middlewares::{
        astek::{self, create_astek},
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

/// Register a new astek using its email
#[post("/", data = "<req>")]
async fn register_asteks(
    _user: AuthenticatedUser,
    req: Json<Request<String>>,
    state: &State<IntrastekState>,
) -> Response<String, String> {
    create_astek(&req.data, &state.db)
        .await
        .map(|a| a.id)
        .into()
}

/// Get all asteks registered
#[get("/")]
async fn get_asteks(state: &State<IntrastekState>) -> Response<Vec<db::astek::Data>, String> {
    astek::get_asteks(&state.db).await.into()
}

/// Get a specific astek by its id
#[get("/<id>")]
async fn get_astek(id: Uuid, state: &State<IntrastekState>) -> Response<db::astek::Data, String> {
    astek::get_astek(&state.db, id).await.into()
}

/// Add an indisponibility to a specific astek
#[post("/<id>", data = "<req>")]
async fn add_indisponibility(
    id: Uuid,
    req: Json<Request<Indisponibility>>,
    state: &State<IntrastekState>,
) -> Response<i32, String> {
    astek::indisponibility::add_indisponibility_to_astek(id, &req.data, &state.db)
        .await
        .into()
}

/// Delete a specific astek by its id
#[delete("/<id>")]
async fn delete_astek(id: Uuid, state: &State<IntrastekState>) -> Response<&'static str, String> {
    astek::delete_astek(&state.db, id)
        .await
        .map(|_| "Ok")
        .into()
}

/// Delete an indisponibility from a specific astek
#[delete("/<id>/indisponibilities/<indisponibility_id>")]
async fn delete_indisponibility(
    id: Uuid,
    indisponibility_id: i32,
    state: &State<IntrastekState>,
) -> Response<&'static str, String> {
    astek::indisponibility::remove_indisponibility_from_astek(id, indisponibility_id, &state.db)
        .await
        .map(|_| "Ok")
        .into()
}
