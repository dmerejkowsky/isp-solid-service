use db::UserData;
use std::time::Instant;

pub struct User {
    pub data: Box<UserData>,
}

impl User {
    pub fn new(data: UserData) -> Self {
        User {
            data: Box::new(data),
        }
    }

    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn address(&self) -> String {
        let data = &self.data;
        format!("{}\n{}", &data.address1, &data.address2)
    }

    pub fn ldap_login(&self) -> String {
        let data = &self.data;
        format!("{}/admin/{}", &data.enterprise_name, &self.name())
    }

    pub fn days_left(&self) -> u64 {
        let data = &self.data;
        let now = Instant::now();
        let created_at: Instant = data.created_at.as_instant();
        let elapsed_secs = (now - created_at).as_secs();
        elapsed_secs / 24 / 60 / 60
    }
}
