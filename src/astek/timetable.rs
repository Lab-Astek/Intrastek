use std::collections::HashMap;

use crate::activity::Activities;

#[derive(Debug, Clone, Default)]
pub(super) struct Timetable {
    total_hours: i128,
    count_per_activity: HashMap<Activities, i128>,
}

impl Timetable {
    pub(super) fn add_time(&mut self, activity: Activities, hours: i128) {
        self.total_hours += hours;
        let count = self.count_per_activity.entry(activity).or_insert(0);
        *count += hours;
    }

    pub(super) fn get_total_hours(&self) -> i128 {
        self.total_hours
    }
}
