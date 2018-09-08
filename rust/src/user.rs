use db::UserData;

pub trait HasData {
    fn data(&self) -> &UserData;
}

pub trait NamedUser {
    fn name(&self) -> String;
}

impl<T> NamedUser for T
where
    T: HasData,
{
    fn name(&self) -> String {
        self.data().name.clone()
    }
}

pub trait AllowAddress {}

pub trait Address {
    fn address(&self) -> String;
}

impl<T> Address for T
where
    T: AllowAddress + HasData,
{
    fn address(&self) -> String {
        let data = &self.data();
        format!("{}\n{}", &data.address1, &data.address2)
    }
}
