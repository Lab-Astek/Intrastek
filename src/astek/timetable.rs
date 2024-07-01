use std::collections::HashMap;

use crate::activity::Activities;

#[derive(Debug, Clone, Default)]
pub(super) struct Timetable {
    pub(super) total_assgin: f64,
    pub(super) count_per_activity: HashMap<Activities, f64>,
}

impl Timetable {
    pub(super) fn add_time(&mut self, activity: Activities, assign: f64) {
        self.total_assgin += assign;
        let count = self.count_per_activity.entry(activity).or_insert(0.0f64);
        *count += assign;
    }

    pub(super) fn get_total_assign(&self) -> f64 {
        self.total_assgin
    }
}
