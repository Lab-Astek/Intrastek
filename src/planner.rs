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

    fn get_available_asteks<'a>(
        &self,
        asteks: &'a Vec<Astek>,
        activity: Activity,
    ) -> Vec<&'a Astek> {
        let mut available_asteks: Vec<&Astek> = Vec::new();

        asteks.iter().for_each(|astek| {
            if astek.is_available(activity.start, activity.end) {
                available_asteks.push(astek);
            }
        });

        available_asteks
    }

    fn pick_asteks<'a>(
        &self,
        activity: &Activity,
        available_asteks: Vec<&'a Astek>,
    ) -> Result<(), String> {
        for i in 0..activity.needed_asteks {
            match available_asteks.get(i as usize) {
                Some(astek) => {
                    println!("Assigning {:?} to {:?}", activity, astek);
                    // astek.assign(activity.clone());
                }
                None => {
                    println!("No astek available for {:?}", activity);
                    return Err("No astek available".to_string())?;
                }
            }
        }
        Ok(())
    }

    pub fn compute(&mut self, asteks: Vec<Astek>) {
        self.activities.iter().for_each(|activity| {
            let available_asteks = self.get_available_asteks(&asteks, activity.clone());
            println!(
                "Available asteks for {:?}: {:?}",
                activity, available_asteks
            );
            self.pick_asteks(activity, available_asteks).unwrap();
        });
    }
}
