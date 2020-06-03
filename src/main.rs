use chrono::DateTime;
use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use log;

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
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbosity")
            .multiple(true)
            .help("Sets the level of verbosity")
        )
        .get_matches();

    loggerv::init_with_verbosity(matches.occurrences_of("verbosity")).unwrap();

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
                Err(e) => log::error!("Gist creation failed:\n\t{:?}", e),
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
            let gists = gstm::list::list(user, since).await;
            match gists {
                Ok(gs) => {
                    for g in gs {
                        // TODO Accurate method of printing w/ variable length truncation
                        let description = match g.description {
                            Some(d) => {
                                let mut desc = d.replace("\n", " ");
                                let max_description_length = {
                                    if let Some((w, _)) = term_size::dimensions() {
                                        w / 3
                                    } else {
                                        40
                                    }
                                };
                                if desc.len() > max_description_length {
                                    desc.truncate(max_description_length);
                                    desc.push_str("...");
                                }
                                desc
                            }
                            _ => String::new(),
                        };

                        let username = match g.owner {
                            Some(o) => o.login,
                            _ => String::new(),
                        };

                        println!("{} {} {} {}", g.created_at, username, g.id, description);
                    }
                }
                Err(e) => log::error!("Retrieving gist listing failed:\n\t{:?}", e),
            }
        }
        _ => {}
    }
}
