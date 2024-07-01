use std::fs::{read_to_string, write};

use crate::{activity::Activity, astek::Astek};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Deserialize, Serialize)]
pub struct Planner {
    activities: Vec<Activity>,
}

impl Planner {
    pub fn new() -> Self {
        Planner {
            activities: Vec::new(),
        }
    }

    pub fn from_file(file: &str) -> Result<Self, String> {
        let file = read_to_string(file).map_err(|e| e.to_string())?;
        let planner: Planner = from_str(&file).map_err(|e| e.to_string())?;
        Ok(planner)
    }

    pub fn save_to_file(self, path: &str) -> Result<(), String> {
        let file = to_string(&self).map_err(|e| e.to_string())?;
        write(path, file).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_activity(&mut self, activity: Activity) {
        self.activities.push(activity);
    }

    pub fn compute(&self, asteks: Vec<Astek>) {
        self.activities.iter().for_each(|activity| {
            asteks.iter().for_each(|astek| {
                if astek.is_available(activity.start, activity.end) {
                    println!(
                        "{} is available for {:?} at {}",
                        astek.name, activity.activity, activity.location
                    );
                }
            });
        });
    }
}
