use db::UserData;

pub struct Member {
    data: UserData,
}

impl Member {
    pub fn new(data: UserData) -> Member {
        Member { data }
    }

    pub fn name(&self) -> String {
        self.data.name.clone()
    }

    pub fn address(&self) -> String {
        let data = &self.data;
        format!("{}\n{}", &data.address1, &data.address2)
    }
}
