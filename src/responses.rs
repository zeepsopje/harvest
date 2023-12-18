use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntries {
    pub time_entries: Vec<TimeEntry>,
}
