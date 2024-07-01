use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Indisponibility {
    start: DateTime<Local>,
    end: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Astek {
    name: String,
    indisponibilities: Vec<Indisponibility>,
}
