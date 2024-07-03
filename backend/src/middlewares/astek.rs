use std::sync::{Arc, Mutex, RwLock};

use log::error;
use rocket::State;
use uuid::Uuid;

use crate::{
    astek::Astek,
    helpers::{InternalError, IntrastekError, NotFound},
    state::IntrastekState,
};

use super::get_state;

pub fn get_astek_and_then<T>(
    id: Uuid,
    state: &State<Mutex<IntrastekState>>,
    callback: impl Fn(&Arc<RwLock<Astek>>) -> Result<T, Box<dyn IntrastekError>>,
) -> Result<T, Box<dyn IntrastekError>> {
    get_state(state, |mutex| {
        if let Some(astek) = mutex
            .asteks
            .iter()
            .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
        {
            callback(astek)
        } else {
            Err(Box::new(NotFound { data: id }))
        }
    })
}

pub fn get_astek(
    id: Uuid,
    state: &State<Mutex<IntrastekState>>,
) -> Result<Astek, Box<dyn IntrastekError>> {
    get_astek_and_then(id, state, |astek| {
        if let Ok(astek) = astek.as_ref().read() {
            Ok(astek.clone())
        } else {
            error!("Failed to read astek");
            Err(Box::new(InternalError))
        }
    })
}
