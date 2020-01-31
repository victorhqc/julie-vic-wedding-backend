use crate::models::NewUser;

pub trait Profile {
    fn new_user(&self) -> NewUser;
}
