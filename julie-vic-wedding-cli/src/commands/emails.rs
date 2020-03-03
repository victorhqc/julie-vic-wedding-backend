use diesel::prelude::*;
use diesel::PgConnection;
use julie_vic_wedding_core::attend_status_type::AttendStatus;
// use julie_vic_wedding_core::models::{ConfirmedUser, User};
// use julie_vic_wedding_core::schema::*;
use uuid::Uuid;

pub fn send_rsvp_emails(conn: PgConnection) {
    println!("Sending RSVP Emails!");

    let user_list: Vec<(String, String, Option<String>, Uuid)> = {
        use julie_vic_wedding_core::schema::confirmed_users::dsl::*;
        use julie_vic_wedding_core::schema::users::dsl::*;

        users
            .inner_join(confirmed_users)
            .select((email, name, last_name, user_id))
            .filter(will_attend.ne(AttendStatus::No))
            .load(&conn)
            .unwrap()
    };

    for user in user_list.iter() {
        let last_name = match &user.2 {
            Some(l) => l,
            None => "",
        };

        let full_name = format!("{} {}", user.1, last_name);
        let email = user.0.clone();
        println!("Will send e-mail for {} ({})", full_name, email);
    }
}
