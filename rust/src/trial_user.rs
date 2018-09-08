use db::UserData;
use std::time::Instant;
use user::{AllowAddress, HasData};

pub struct TrialUser {
    data: UserData,
}

impl HasData for TrialUser {
    fn data(&self) -> &UserData {
        &self.data
    }
}

impl AllowAddress for TrialUser {}

impl TrialUser {
    pub fn new(data: UserData) -> TrialUser {
        TrialUser { data }
    }

    pub fn name(&self) -> &str {
        &self.data.temp_login
    }

    pub fn days_left(&self) -> u64 {
        let data = &self.data;
        let now = Instant::now();
        let created_at: Instant = data.created_at.as_instant();
        let elapsed_secs = (now - created_at).as_secs();
        elapsed_secs / 24 / 60 / 60
    }
}
