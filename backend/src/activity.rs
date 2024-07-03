use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{interval::Interval, module::Module, routes::activities::ActivityRequest};
use log::info;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum Activities {
    FollowUp,
    Bootstrap,
    Review,
    Keynote,
    Surveillance,
    Permanence,
}

impl Display for Activities {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Activities::FollowUp => write!(f, "Follow Up"),
            Activities::Bootstrap => write!(f, "Bootstrap"),
            Activities::Review => write!(f, "Review"),
            Activities::Keynote => write!(f, "Keynote"),
            Activities::Surveillance => write!(f, "Surveillance"),
            Activities::Permanence => write!(f, "Permanence"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    pub id: Uuid,
    pub activity: Activities,
    pub interval: Interval,
    pub location: String,
    pub needed_asteks: u32,
    pub module: Option<Module>,
    pub asteks: Vec<Uuid>,
}

impl Activity {
    pub fn add_astek(&mut self, astek: Uuid) {
        self.asteks.push(astek);
    }
}

impl From<ActivityRequest> for Activity {
    fn from(value: ActivityRequest) -> Self {
        let id = Uuid::new_v4();
        info!("Creating activity with id: {}", id);
        Activity {
            id,
            activity: value.activity,
            interval: value.interval,
            location: value.location,
            needed_asteks: value.needed_asteks,
            module: value.module,
            asteks: Vec::new(),
        }
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(module) = &self.module {
            writeln!(f, "{} {}:", self.activity, module)?;
        } else {
            writeln!(f, "{}:", self.activity)?;
        }
        writeln!(f, "\tDate: {}", self.interval.start.date_naive())?;
        writeln!(f, "\tAsteks:")?;
        if self.asteks.is_empty() {
            writeln!(f, "\t\t- None")?;
        }
        self.asteks
            .iter()
            .try_for_each(|astek| writeln!(f, "\t\t- {}", astek))
    }
}
