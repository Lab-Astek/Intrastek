use std::sync::{Arc, Mutex, RwLock};

use log::error;
use rocket::State;
use uuid::Uuid;

use crate::{
    astek::Astek,
    db::{astek, PrismaClient},
    helpers::{InternalError, IntrastekError, NotFound},
    state::IntrastekState,
};

use super::get_state;
use crate::db;

// pub fn get_astek_and_then<T>(
//     id: Uuid,
//     state: &State<Mutex<IntrastekState>>,
//     callback: impl Fn(&Arc<RwLock<Astek>>) -> Result<T, Box<dyn IntrastekError>>,
// ) -> Result<T, Box<dyn IntrastekError>> {
//     get_state(state, |mutex| {
//         if let Some(astek) = mutex
//             .asteks
//             .iter()
//             .find(|a| a.as_ref().read().is_ok_and(|x| x.id == id))
//         {
//             callback(astek)
//         } else {
//             Err(Box::new(NotFound { data: id }))
//         }
//     })
// }

// pub fn get_astek(
//     id: Uuid,
//     state: &State<Mutex<IntrastekState>>,
// ) -> Result<Astek, Box<dyn IntrastekError>> {
//     get_astek_and_then(id, state, |astek| {
//         if let Ok(astek) = astek.as_ref().read() {
//             Ok(astek.clone())
//         } else {
//             error!("Failed to read astek");
//             Err(Box::new(InternalError))
//         }
//     })
// }

pub async fn create_astek(
    email: &String,
    db: &Arc<PrismaClient>,
) -> Result<db::astek::Data, Box<dyn IntrastekError>> {
    let new_uuid = uuid::Uuid::new_v4();

    match db
        .astek()
        .create(new_uuid.to_string(), email.clone(), vec![])
        .exec()
        .await
    {
        Ok(new) => Ok(new),
        Err(e) => {
            error!("Failed to create astek: {e}");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_asteks(
    db: Arc<PrismaClient>,
) -> Result<Vec<db::astek::Data>, Box<dyn IntrastekError>> {
    match db.astek().find_many(vec![]).exec().await {
        Ok(asteks) => Ok(asteks),
        Err(_) => {
            error!("Failed to get asteks");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_astek(
    db: Arc<PrismaClient>,
    uuid: Uuid,
) -> Result<astek::Data, Box<dyn IntrastekError>> {
    match db
        .astek()
        .find_unique(astek::id::equals(uuid.to_string()))
        .exec()
        .await
    {
        Ok(Some(astek)) => Ok(astek),
        _ => {
            error!("Failed to get astek {uuid}");
            Err(Box::new(NotFound { data: uuid }))
        }
    }
}
