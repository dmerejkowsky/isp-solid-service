use user::User;

pub struct Admin {
    user: User,
}

impl Admin {
    pub fn new(user: User) -> Admin {
        Admin { user }
    }

    pub fn ldap_login(&self) -> String {
        self.user.ldap_login()
    }
}
