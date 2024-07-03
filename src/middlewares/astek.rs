use std::sync::{Arc, Mutex, RwLock};

use log::error;
use rocket::State;
use uuid::Uuid;

use crate::{astek::Astek, helpers::IntrastekErrors, state::IntrastekState};

pub fn get_astek_and_then<T>(
    id: Uuid,
    state: &State<Mutex<IntrastekState>>,
    callback: impl Fn(&Arc<RwLock<Astek>>) -> Result<T, IntrastekErrors>,
) -> Result<T, IntrastekErrors> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(astek) = mutex
                .asteks
                .iter()
                .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                callback(astek)
            } else {
                Err(IntrastekErrors::NotFound(id))
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(IntrastekErrors::InternalError)
        }
    }
}

pub fn get_astek(id: Uuid, state: &State<Mutex<IntrastekState>>) -> Result<Astek, IntrastekErrors> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(astek) = mutex
                .asteks
                .iter()
                .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
            {
                match astek.as_ref().read() {
                    Ok(astek) => Ok(astek.clone()),
                    Err(_) => Err(IntrastekErrors::InternalError),
                }
            } else {
                Err(IntrastekErrors::NotFound(id))
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(IntrastekErrors::InternalError)
        }
    }
}
