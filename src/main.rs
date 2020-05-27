use chrono::DateTime;
use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use gstm;

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
                    Arg::with_name("private")
                        .short("-p")
                        .long("--private")
                        .help("Make your new Gist private"),
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
                )
                // .arg( TODO implement pagination
                //     Arg::with_name("count")
                //         .short("-c")
                //         .long("--count")
                //         .takes_value(true)
                //         .default_value("3000")
                //         .help("Retrieve [count] many values."),
                // )
        ])
        .get_matches();

    match matches.subcommand() {
        ("create", Some(sc)) => {
            let files: Vec<String> = sc.values_of("files").unwrap().map(String::from).collect();
            let is_public = sc.is_present("private");
            let description = match sc.value_of("description") {
                Some(s) => Some(String::from(s)),
                None => None,
            };
            let res = gstm::create(files, is_public, description).await;
            match res {
                Ok(value) => println!("Gist available at {}", value.html_url),
                Err(e) => println!("An error occurred:\n\t{:?}", e),
            };
        }
        ("list", Some(sc)) => {
            let user = match sc.value_of("user") {
                Some(s) => Some(String::from(s)),
                None => None,
            };
            let since = match sc.value_of("since") {
                Some(s) => Some(DateTime::parse_from_rfc3339(s).unwrap()),
                None => None,
            };
            // let count = match sc.value_of("count") {
            //     Some(s) => Some(s.parse::<i32>().unwrap()),
            //     None => None,
            // };
            gstm::list(user, since).await;
        }
        _ => {}
    }
}
