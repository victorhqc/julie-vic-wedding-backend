use oauth2::{
    basic::{BasicTokenType},
    EmptyExtraTokenFields, StandardTokenResponse,
};

use crate::models::NewUser;

pub trait Profile {
    fn new_user(&self) -> NewUser;
}

pub type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub trait Code {
    fn code(&self) -> String;
}
