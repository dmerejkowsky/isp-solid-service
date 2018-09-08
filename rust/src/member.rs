use db::UserData;
use user::AllowAddress;
use user::HasData;

pub struct Member {
    data: UserData,
}

impl AllowAddress for Member {}

impl HasData for Member {
    fn data(&self) -> &UserData {
        &self.data
    }
}

impl Member {
    pub fn new(data: UserData) -> Member {
        Member { data }
    }
}
