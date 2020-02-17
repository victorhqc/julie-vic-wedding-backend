use julie_vic_wedding_core::models::NewUser;
// use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};

pub trait Profile {
    fn new_user(&self) -> NewUser;
}

// pub type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub trait Code {
    fn code(&self) -> String;
}
