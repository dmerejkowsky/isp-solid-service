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
