use db::UserData;
use user::HasData;

pub struct Member {
    data: UserData,
}

impl HasData for Member {
    fn data(&self) -> &UserData {
        &self.data
    }
}

impl Member {
    pub fn new(data: UserData) -> Member {
        Member { data }
    }

    pub fn address(&self) -> String {
        let data = &self.data;
        format!("{}\n{}", &data.address1, &data.address2)
    }
}
