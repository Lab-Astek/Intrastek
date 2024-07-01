use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    fs::{read_to_string, write},
    rc::Rc,
};

use crate::{
    activity::{Activities, Activity},
    astek::astek::Astek,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Deserialize, Serialize)]
pub struct Planner {
    activities: Vec<Activity>,
}

fn sort_asteks_by_time_on_activities(
    asteks: &Vec<Rc<RefCell<Astek>>>,
    activity: Activities,
) -> Vec<Rc<RefCell<Astek>>> {
    let mut asteks = asteks.clone();
    asteks.sort_by(|a, b| {
        let a = a.borrow().get_time_spent_for_activity(activity.clone());
        let b = b.borrow().get_time_spent_for_activity(activity.clone());
        match a.partial_cmp(&b) {
            Some(order) => order,
            None => Ordering::Equal,
        }
    });
    asteks
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

    fn get_available_asteks(
        asteks: &Vec<Rc<RefCell<Astek>>>,
        activity: &Activity,
    ) -> Vec<Rc<RefCell<Astek>>> {
        let mut available_asteks: Vec<Rc<RefCell<Astek>>> = Vec::new();

        asteks.iter().for_each(|astek| {
            if astek
                .as_ref()
                .borrow()
                .is_available(activity.start, activity.end)
            {
                available_asteks.push(astek.clone());
            }
        });

        available_asteks
    }

    fn pick_asteks<'a>(
        activity: &mut Activity,
        available_asteks: Vec<Rc<RefCell<Astek>>>,
    ) -> Result<(), String> {
        for i in 0..activity.needed_asteks {
            match available_asteks.get(i as usize) {
                Some(astek) => {
                    astek.borrow_mut().assign(activity.clone());
                    activity.add_astek(astek.borrow().name.clone());
                }
                None => {
                    return Err("No astek available".to_string())?;
                }
            }
        }
        Ok(())
    }

    pub fn compute(&mut self, asteks: &Vec<Rc<RefCell<Astek>>>) -> Result<(), String> {
        self.activities.iter_mut().try_for_each(|activity| {
            let available_asteks = Planner::get_available_asteks(asteks, activity);
            Planner::pick_asteks(activity, available_asteks)
        })
    }
}

impl Display for Planner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.activities
            .iter()
            .try_for_each(|activity| write!(f, "{}", activity))
    }
}
