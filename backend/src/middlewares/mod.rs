use std::sync::Mutex;

use rocket::State;

use crate::{
    helpers::{InternalError, IntrastekError},
    state::IntrastekState,
};

pub mod astek;

pub fn get_state<T>(
    state: &State<Mutex<IntrastekState>>,
    f: impl Fn(&IntrastekState) -> Result<T, Box<dyn IntrastekError>>,
) -> Result<T, Box<dyn IntrastekError>> {
    match state.lock() {
        Ok(mutex) => f(&mutex),
        Err(_) => Err(Box::new(InternalError)),
    }
}

pub fn get_state_mut<T>(
    state: &State<Mutex<IntrastekState>>,
    f: impl Fn(&mut IntrastekState) -> Result<T, Box<dyn IntrastekError>>,
) -> Result<T, Box<dyn IntrastekError>> {
    match state.lock() {
        Ok(mut mutex) => f(&mut mutex),
        Err(_) => Err(Box::new(InternalError)),
    }
}
