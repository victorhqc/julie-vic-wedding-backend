use crate::auth::Profile;
use crate::db::Repo;
use julie_vic_wedding_core::attend_status_type::AttendStatus;
use julie_vic_wedding_core::models::{ConfirmedUser, NewConfirmedUser, Token, User};
use julie_vic_wedding_core::schema::*;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::Future;
use uuid::Uuid;

pub fn find_by_email(
    repo: Repo,
    user_email: String,
) -> impl Future<Item = User, Error = DieselError> {
    use julie_vic_wedding_core::schema::users::dsl::*;
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
            use julie_vic_wedding_core::schema::users::dsl::*;

            users
                .filter(email.eq(new_user.email.clone()))
                .first::<User>(&conn)
        };

        match user {
            Ok(u) => Ok(u),
            Err(_) => diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(&conn),
        }
    })
}

pub struct NewUserData {
    pub user_id: Uuid,
    pub will_attend: AttendStatus,
    pub table_id: Option<Uuid>,
    pub token: String,
}

pub fn rsvp_confirmation(
    repo: Repo,
    data: NewUserData,
) -> impl Future<Item = ConfirmedUser, Error = DieselError> {
    repo.run(move |conn| {
        let existing = {
            use julie_vic_wedding_core::schema::confirmed_users::dsl::*;

            diesel::update(confirmed_users.find(data.user_id))
                .set(will_attend.eq(&data.will_attend))
                .get_result::<ConfirmedUser>(&conn)
        };

        match existing {
            Ok(e) => Ok(e),
            Err(_) => {
                let token = {
                    use julie_vic_wedding_core::schema::tokens::dsl::*;
                    tokens.filter(token.eq(data.token)).first::<Token>(&conn)
                };

                match token {
                    Ok(t) => {
                        let confirmed_user = NewConfirmedUser {
                            user_id: data.user_id,
                            will_attend: data.will_attend,
                            table_id: data.table_id,
                            token_id: t.id,
                        };

                        diesel::insert_into(confirmed_users::table)
                            .values(&confirmed_user)
                            .get_result(&conn)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    })
}
