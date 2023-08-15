use chrono::{DateTime, Utc};


pub enum CleaningTaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

pub struct CleaningTaskEntity {
    pub id: u32,
    pub description: String,
    pub status: CleaningTaskStatus,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}