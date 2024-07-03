use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Module {
    Cpe,
    Psu,
    Mul,
    Mat,
    Web,
    Aia,
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Module::Cpe => write!(f, "CPE"),
            Module::Psu => write!(f, "PSU"),
            Module::Mul => write!(f, "MUL"),
            Module::Mat => write!(f, "MAT"),
            Module::Web => write!(f, "WEB"),
            Module::Aia => write!(f, "AIA"),
        }
    }
}
