use anyhow::Result;
use diesel::{RunQueryDsl, insert_into};
use diesel::prelude::*;

use uuid::Uuid;

use crate::models::{User, NewUser};
use crate::api::{DBConnection};
use crate::auth::GoogleProfile;

pub fn create_user(conn: &DBConnection, profile: &GoogleProfile) -> Result<User> {
    use crate::schema::users;

    let id = Uuid::new_v4();

    let new_user = NewUser {
        id,
        email: profile.email.clone(),
        name: profile.given_name.as_ref().unwrap_or(&String::from("")).to_string(),
        last_name: profile.family_name.clone(),
    };

    let result = insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating user");

    Ok(result)
}

pub fn find_by_email(conn: &DBConnection, user_email: String) -> Result<User> {
    use crate::schema::users::dsl::*;

    let found_user = users
        .filter(email.eq(user_email))
        .first::<User>(conn)
        .expect("Error finding users");

    Ok(found_user)
}
