use user::User;

pub struct Member {
    user: User,
}

impl Member {
    pub fn new(user: User) -> Member {
        Member { user }
    }

    pub fn address(&self) -> String {
        let data = &self.user.data;
        format!("{}\n{}", &data.address1, &data.address2)
    }
}
