use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntry {
    pub id: u64,
    pub spent_date: String,
    pub hours: f64,
    pub notes: String,
    pub is_running: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntries {
    pub time_entries: Vec<TimeEntry>,
}
