use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};

use gstm_cli::{handle_create_command, handle_get_command, handle_list_command};

#[tokio::main]
async fn main() {
    let matches = App::new("gstm")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Gist manipulator")
        .subcommands(vec![
            SubCommand::with_name("create")
                .about("Create a new Gist")
                .arg(Arg::with_name("files").multiple(true).required(true))
                .arg(
                    Arg::with_name("public")
                        .short("-p")
                        .long("--public")
                        .help("Make your new Gist public"),
                )
                .arg(
                    Arg::with_name("description")
                        .short("-d")
                        .long("--description")
                        .takes_value(true)
                        .help("The description of your Gist"),
                ),
            SubCommand::with_name("list")
                .about("Retrieve a listing of Gists")
                .arg(
                    Arg::with_name("user")
                        .short("-u")
                        .long("--user")
                        .takes_value(true)
                        .help("Filter by username"),
                )
                .arg(
                    Arg::with_name("since")
                        .short("-s")
                        .long("--since")
                        .takes_value(true)
                        .help("Limit to Gists uploaded after an RFC 3339 (ISO 8601) timestamp (YYYY-MM-DDTHH:MM:SSZ)"),
                ),
                // .arg( TODO implement pagination
                //     Arg::with_name("count")
                //         .short("-c")
                //         .long("--count")
                //         .takes_value(true)
                //         .default_value("3000")
                //         .help("Retrieve [count] many values."),
                // )
            SubCommand::with_name("get")
                .about("Retrieve the content of a single Gist")
                .arg(
                    Arg::with_name("id")
                    .required(true)
                    .help("The ID of the given Gist")
                )
                .arg(
                    Arg::with_name("greedy")
                    .short("-g")
                    .long("--greedy")
                    .help("Attempt to retrieve files larger than 1MB in size")
                )
                .arg(
                    Arg::with_name("output")
                    .short("-o")
                    .long("--output")
                    .takes_value(true)
                    .help("Write to a directory, rather than stdout. Implies -g")
                )
                .arg(
                    Arg::with_name("delimiter")
                    .short("-d")
                    .long("--delimiter")
                    .takes_value(true)
                    .help("Optional delimiter between file text, if sent to stdout")
                )
                .arg(
                    Arg::with_name("no-content")
                    .short("-c")
                    .long("--no-content")
                    .help("Hide the text of any file from output; meta only")
                )
        ])
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbosity")
            .multiple(true)
            .help("Sets the level of verbosity")
        )
        .get_matches();

    loggerv::init_with_verbosity(matches.occurrences_of("verbosity")).unwrap();

    handle_matches(matches).await;
}

async fn handle_matches(matches: ArgMatches<'_>) {
    match matches.subcommand() {
        ("create", Some(sc)) => handle_create_command(sc).await,
        ("list", Some(sc)) => handle_list_command(sc).await,
        ("get", Some(sc)) => handle_get_command(sc).await,
        _ => {}
    }
}
