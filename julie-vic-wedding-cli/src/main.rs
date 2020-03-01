use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

fn main() {
    println!("Hello world");
}

struct Cli<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Cli<'a, 'b> {
    pub fn new() -> Self {
        let app = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("generate_tokens")
                    .about("Generates random tokens for user registration")
                    .arg(
                        Arg::with_name("amount")
                            .short("a")
                            .long("amount")
                            .takes_value(true)
                            .help("Amount of tokens to create"),
                    ),
            );
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

        Self { app }
    }
}
