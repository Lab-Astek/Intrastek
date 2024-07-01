mod indisponibility;
pub mod timetable;

use std::fmt::{self, Display, Formatter};

use indisponibility::{Indisponibility, IndisponibilityType};
use log::info;
use serde::{Deserialize, Serialize};
use timetable::Timetable;

use crate::{
    activity::{Activities, Activity},
    interval::Interval,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Astek {
    pub name: String,
    indisponibilities: Vec<Indisponibility>,
    assignations: Vec<Activity>,
    #[serde(skip_deserializing, skip_serializing)]
    timetable: Timetable,
}

impl Astek {
    pub fn new(name: &str) -> Self {
        info!("Creating astek: {}", name);
        Astek {
            name: name.to_string(),
            indisponibilities: Vec::new(),
            assignations: Vec::new(),
            timetable: Timetable::default(),
        }
    }

    pub fn add_indisponibility(&mut self, start: &str, end: &str) {
        self.indisponibilities.push(Indisponibility::new(
            Interval::new(start, end),
            IndisponibilityType::Private,
        ));
    }

    pub fn is_available(&self, act_interval: &Interval) -> bool {
        self.indisponibilities
            .iter()
            .all(|indisponibility| !act_interval.intersects(indisponibility.get_interval()))
    }

    pub fn assign(&mut self, activity: Activity) {
        let time = (activity.interval.end.timestamp() - activity.interval.start.timestamp()) / 3600;
        self.indisponibilities.push(Indisponibility::new(
            activity.interval.clone(),
            IndisponibilityType::Activity(activity.activity.clone()),
        ));
        self.timetable.add_time(
            activity.activity.clone(),
            if time <= 4i64 { 0.5f64 } else { 1.0f64 },
        );
        self.assignations.push(activity);
    }

    pub fn get_time_spent_for_activity(&self, activity: Activities) -> f64 {
        self.timetable
            .count_per_activity
            .get(&activity)
            .copied()
            .unwrap_or(0.0f64)
    }
}

impl Display for Astek {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "Indisponibilities:")?;
        self.indisponibilities
            .iter()
            .try_for_each(|indisponibility| writeln!(f, "\t- {}", indisponibility))?;
        writeln!(f, "Assignations:")?;
        // self.assignations
        //     .iter()
        //     .try_for_each(|activity| writeln!(f, "\t- {}", activity))?;
        write!(
            f,
            "{} assignations since start of the year",
            self.timetable.get_total_assign()
        )
    }
}
