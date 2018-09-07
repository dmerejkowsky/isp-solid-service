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

pub struct User<'a> {
    data: &'a UserData,
}

impl<'a> User<'a> {
    pub fn new(data: &'a UserData) -> Self {
        User { data: data }
    }

    pub fn name(&self) -> &str {
        if self.data.trial {
            &self.data.temp_login
        } else {
            &self.data.name
        }
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
        let created_at: Instant = data.created_at.get();
        let elapsed_secs = (now - created_at).as_secs();
        elapsed_secs / 24 / 60 / 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn has_a_name() {
        let mut data: UserData = Default::default();
        data.name = "john".into();
        let user = User::new(&data);
        assert_eq!(user.name(), "john");
    }

    #[test]
    fn member_has_an_address() {
        let mut data: UserData = Default::default();
        data.address1 = "452 Wilson Summit".into();
        data.address2 = "East Dawnshier, AK 96919".into();
        let user = User::new(&data);
        assert_eq!(
            user.address(),
            "452 Wilson Summit\nEast Dawnshier, AK 96919"
        );
    }

    #[test]
    fn admin_has_a_ldap_login() {
        let mut data: UserData = Default::default();
        data.name = "john".into();
        data.enterprise_name = "fooCorp".into();
        data.admin = true;
        let user = User::new(&data);
        assert_eq!(user.ldap_login(), "fooCorp/admin/john");
    }

    #[test]
    fn trial_user_has_a_temp_name() {
        let mut data: UserData = Default::default();
        data.trial = true;
        data.temp_login = "temp login".into();
        let user = User::new(&data);
        assert_eq!(user.name(), "temp login");
    }

    #[test]
    fn trial_user_has_a_number_of_days_left() {
        let mut data: UserData = Default::default();
        data.trial = true;
        let now = Instant::now();
        let two_days_ago = now - Duration::from_secs(60 * 60 * 24 * 2);
        data.created_at = two_days_ago.into();
        let user = User::new(&data);
        assert_eq!(user.days_left(), 2);
    }

}
