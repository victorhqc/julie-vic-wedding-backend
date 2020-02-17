use crate::models::NewUser;

pub trait Profile {
    fn new_user(&self) -> NewUser;
}

pub trait Code {
    fn code(&self) -> String;
}
