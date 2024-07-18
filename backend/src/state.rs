use std::sync::Arc;

use crate::db::{self};

#[derive(Debug, Clone)]
pub struct IntrastekState {
    pub db: Arc<db::PrismaClient>,
}

impl IntrastekState {
    pub async fn default() -> Self {
        IntrastekState {
            db: Arc::new(db::new_client().await.expect("Failed to create db client")),
        }
    }
}
