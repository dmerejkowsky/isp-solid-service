mod db;

pub mod user;


#[cfg(test)]
mod tests {
    use super::db::UserData;
    use super::user::User;
    use std::time::{Duration,Instant};

    #[test]
    fn has_a_name() {
        let mut data: UserData = Default::default();
        data.name = "john".into();
        let user = User::new(data);
        assert_eq!(user.name(), "john");
    }

    #[test]
    fn member_has_an_address() {
        let mut data: UserData = Default::default();
        data.address1 = "452 Wilson Summit".into();
        data.address2 = "East Dawnshier, AK 96919".into();
        let user = User::new(data);
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
        let user = User::new(data);
        assert_eq!(user.ldap_login(), "fooCorp/admin/john");
    }

    #[test]
    fn trial_user_has_a_temp_name() {
        let mut data: UserData = Default::default();
        data.trial = true;
        data.temp_login = "temp login".into();
        let user = User::new(data);
        assert_eq!(user.name(), "temp login");
    }

    #[test]
    fn trial_user_has_a_number_of_days_left() {
        let mut data: UserData = Default::default();
        data.trial = true;
        let now = Instant::now();
        let two_days_ago = now - Duration::from_secs(60 * 60 * 24 * 2);
        data.created_at = two_days_ago.into();
        let user = User::new(data);
        assert_eq!(user.days_left(), 2);
    }

}
