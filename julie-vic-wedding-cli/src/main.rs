use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use dotenv::dotenv;

mod commands;
mod db;

use crate::commands::{emails, tokens};

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    env_logger::init();
    let conn = db::establish_connection();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate things")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("tokens")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .about("Generate tokens")
                        .arg(
                            Arg::with_name("amount")
                                .short("a")
                                .long("amount")
                                .takes_value(true)
                                .help("Amount of tokens to create"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("emails")
                .about("Email stuff")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(SubCommand::with_name("rsvp").about("Send RSVP emails")),
        )
        // .subcommand(
        //     SubCommand::with_name("users")
        //         .about("Gets users information")
        //         .arg(
        //             Arg::with_name("filter")
        //                 .short("f")
        //                 .long("filter")
        //                 .takes_value(true)
        //                 .help(
        //                     "Filters users by provided string. Filter will consider name & email"
        //                 ),
        //         )
        //         .arg(
        //             Arg::with_name("confirmed")
        //                 .short("c")
        //                 .long("confirmed")
        //                 .takes_value(false)
        //                 .help("Returns only confirmed users")
        //         ),
        // );
        .get_matches();

    match matches.subcommand() {
        ("generate", Some(generate_matches)) => match generate_matches.subcommand() {
            ("tokens", Some(token_matches)) => {
                let amount: u32 = token_matches
                    .value_of("amount")
                    .unwrap()
                    .parse()
                    .expect("Invalid number");
                tokens::generate_tokens(amount, conn);
            }
            _ => unreachable!(),
        },
        ("emails", Some(email_matches)) => match email_matches.subcommand() {
            ("rsvp", Some(_)) => {
                emails::send_rsvp_emails(conn);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
