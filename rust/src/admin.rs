use db::UserData;
use user::{HasData, NamedUser};

pub struct Admin {
    data: UserData,
}

impl HasData for Admin {
    fn data(&self) -> &UserData {
        &self.data
    }
}

impl Admin {
    pub fn new(data: UserData) -> Admin {
        Admin { data }
    }

    pub fn ldap_login(&self) -> String {
        format!("{}/admin/{}", &self.data.enterprise_name, &self.name())
    }
}
