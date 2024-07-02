use std::sync::{Arc, RwLock};

use crate::{astek::Astek, planner::Planner};

#[derive(Debug, Clone)]
pub struct IntrastekState {
    pub planner: Planner,
    pub asteks: Vec<Arc<RwLock<Astek>>>,
}

impl Default for IntrastekState {
    fn default() -> Self {
        IntrastekState {
            planner: Planner::new(),
            asteks: Vec::new(),
        }
    }
}
