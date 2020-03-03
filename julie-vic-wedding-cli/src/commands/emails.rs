use diesel::prelude::*;
use diesel::PgConnection;
use julie_vic_wedding_core::attend_status_type::AttendStatus;
use std::convert::From;
use std::env;
use uuid::Uuid;

pub fn send_rsvp_emails(conn: PgConnection) {
    println!("Sending RSVP Emails!");
    let domain = env::var("DOMAIN").expect("DOMAIN is not set");

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
        let user_email = user.0.clone();
        println!("Will send e-mail for {} ({})", full_name, user_email);

        let sender_email = format!("rsvp@{}", domain);
        println!("sender: {}", sender_email);

        let email = Email::new(sender_email, user_email, full_name);

        match send_email(&email) {
            Ok(_) => println!("Email to {} was successfully sent", email.to),
            Err(e) => println!("Failed to send email to {}: {:?}", email.from, e),
        };
    }
}

fn send_email(email: &Email) -> Result<(), Box<dyn std::error::Error>> {
    let domain = env::var("DOMAIN").expect("DOMAIN is not set");
    let mailgun_key = env::var("MAILGUN_KEY").expect("MAILGUN_KEY is not set");

    let url = format!("https://api.eu.mailgun.net/v3/{}/messages", domain);

    let form = reqwest::blocking::multipart::Form::new()
        .text("from", email.from.clone())
        .text("to", email.to.clone())
        .text("subject", String::from("Test"))
        .text("text", String::from("Hello World"));

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(&url)
        .basic_auth("api", Some(mailgun_key))
        .multipart(form)
        .send()?;

    println!("{:#?}", res);
    Ok(())
}

struct Email {
    pub from: String,
    pub to: String,
}

impl Email {
    pub fn new(from_email: String, to_email: String, to_name: String) -> Self {
        Email {
            from: from_email,
            to: format!("{} <{}>", to_name, to_email),
        }
    }
}
