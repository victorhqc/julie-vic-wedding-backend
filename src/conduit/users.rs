use crate::auth::Profile;
use crate::models::{ConfirmedUser, NewConfirmedUser, User};
use crate::schema::*;

use crate::db::Repo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::Future;

pub fn find_by_email(
    repo: Repo,
    user_email: String,
) -> impl Future<Item = User, Error = DieselError> {
    use crate::schema::users::dsl::*;
    repo.run(|conn| users.filter(email.eq(user_email)).first::<User>(&conn))
}

// TODO: Reduce code repetition here. I have no idea to implement an elegant solution right now.
// Maybe when moving to "async" is easy...
pub fn find_or_create<T: Profile>(
    repo: Repo,
    profile: T,
) -> impl Future<Item = User, Error = DieselError> {
    let new_user = profile.new_user();

    repo.run(move |conn| {
        let user = {
            use crate::schema::users::dsl::*;

            users
                .filter(email.eq(new_user.email.clone()))
                .first::<User>(&conn)
        };

        match user {
            Ok(u) => Ok(u),
            Err(_) => {
                diesel::insert_into(users::table)
                    .values(&new_user)
                    .get_result(&conn)
            }
        }
    })
}

pub fn rsvp_confirmation(
    repo: Repo,
    confirmed_user: NewConfirmedUser,
) -> impl Future<Item = ConfirmedUser, Error = DieselError> {
    repo.run(move |conn| {
        let existing = {
            use crate::schema::confirmed_users::dsl::*;

            diesel::update(confirmed_users.find(confirmed_user.user_id))
                .set(will_attend.eq(&confirmed_user.will_attend))
                .get_result::<ConfirmedUser>(&conn)
        };

        match existing {
            Ok(e) => Ok(e),
            Err(_) => diesel::insert_into(confirmed_users::table)
                .values(&confirmed_user)
                .get_result(&conn),
        }
    })
}
