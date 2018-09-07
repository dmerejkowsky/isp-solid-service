use user::User;

pub struct TrialUser {
    user: User,
}

impl TrialUser {
    pub fn new(user: User) -> TrialUser {
        TrialUser { user }
    }

    pub fn name(&self) -> &str {
        &self.user.data.temp_login
    }
}
