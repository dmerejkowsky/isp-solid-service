use db::UserData;

pub struct Admin {
    data: UserData,
}

impl Admin {
    pub fn new(data: UserData) -> Admin {
        Admin { data }
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn ldap_login(&self) -> String {
        format!("{}/admin/{}", &self.data.enterprise_name, &self.name())
    }
}
