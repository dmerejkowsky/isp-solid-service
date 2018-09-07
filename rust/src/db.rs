use std::time::Instant;

// Assume this comes from the db
#[derive(Default)]
pub struct UserData {
    pub enterprise_name: String,
    pub name: String,
    pub temp_login: String,
    pub address1: String,
    pub address2: String,
    pub created_at: Time,
    pub admin: bool,
    pub trial: bool,
}

// We need our own type to have a default created_at
pub struct Time {
    instant: Instant,
}

impl Time {
    pub fn new() -> Time {
        Time {
            instant: Instant::now(),
        }
    }
    pub fn get(&self) -> Instant {
        self.instant
    }
}

impl Default for Time {
    fn default() -> Time {
        Time::new()
    }
}

impl From<Instant> for Time {
    fn from(instant: Instant) -> Self {
        Time { instant }
    }
}
