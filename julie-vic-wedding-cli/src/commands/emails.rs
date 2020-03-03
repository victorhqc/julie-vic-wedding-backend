use diesel::prelude::*;
use diesel::PgConnection;
use julie_vic_wedding_core::attend_status_type::AttendStatus;
use lettre::error::Error as LettreError;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, SendableEmail, SmtpClient,
    Transport,
};
use native_tls::{Protocol, TlsConnector};
use std::convert::From;
use std::env;
use thiserror::Error;
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

        let email_from = match EmailAddress::new(sender_email.clone()) {
            Ok(email) => email,
            Err(e) => {
                println!("Failed to send email from {}: {:?}", sender_email, e);
                continue;
            }
        };
        let email_to = match EmailAddress::new(user_email.clone()) {
            Ok(email) => email,
            Err(e) => {
                println!("Failed to send email to {}: {:?}", user_email, e);
                continue;
            }
        };

        let email = SendableEmail::new(
            Envelope::new(Some(email_from), vec![email_to]).unwrap(),
            "Hello".to_string(),
            "Hello world".to_string().into_bytes(),
        );

        match send_email(email) {
            Ok(_) => println!("Email to {} was successfully sent", user_email),
            Err(e) => println!("Failed to send email to {}: {:?}", user_email, e),
        }
    }
}

fn send_email(email: SendableEmail) -> Result<(), EmailErr> {
    let smtp_url = env::var("SMTP_URL").expect("SMTP_URL is not set");
    let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT is not set");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER is not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set");
    let domain = env::var("DOMAIN").expect("DOMAIN is not set");

    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
    let tls_parameters =
        ClientTlsParameters::new(smtp_url.to_string(), tls_builder.build().unwrap());
    let mut mailer = SmtpClient::new(
        (
            smtp_url.as_ref(),
            smtp_port.parse().expect("SMTP_PORT is not a valid number"),
        ),
        ClientSecurity::Required(tls_parameters),
    )
    .unwrap()
    // Set the name sent during EHLO/HELO, default is `localhost`
    .hello_name(ClientId::Domain(domain))
    .authentication_mechanism(Mechanism::Login)
    .credentials(Credentials::new(smtp_user, smtp_password))
    // Enable SMTPUTF8 if the server supports it
    .smtp_utf8(true)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    // Send the email
    let result = mailer.send(email);

    if result.is_ok() {
        Ok(())
    } else {
        Err(EmailErr::FailedToSend)
    }
}

#[derive(Error, Debug)]
pub enum EmailErr {
    #[error("Failed to send email")]
    FailedToSend,
    #[error("Lettre failed: {0}")]
    LettreError(LettreError),
}

impl From<LettreError> for EmailErr {
    fn from(error: LettreError) -> Self {
        EmailErr::LettreError(error)
    }
}
