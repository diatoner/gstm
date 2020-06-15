use std::fs::File;
use std::io::prelude::*;

use chrono::DateTime;
use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};
use github_auth::{Authenticator, Scope};
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

async fn handle_create_command(sc: &clap::ArgMatches<'_>) {
    // Parse input
    let files: Vec<String> = sc.values_of("files").unwrap().map(String::from).collect();
    let is_public: bool = sc.is_present("private");
    let description: Option<String> = sc.value_of("description").map(|x| x.to_string()).take();
    // Collect GitHub Auth token
    let auth = Authenticator::builder("gstm".into())
        .scope(Scope::Gist)
        .build();
    let token = auth.auth().unwrap();
    log::info!("Token stored at {:?}", auth.location());
    // Process parsed input
    let res = gstm::create(files, is_public, description, token.into_string()).await;
    // Print output
    match res {
        Ok(value) => println!("Gist available at {}", value.html_url.unwrap()),
        Err(e) => log::error!("Gist creation failed: {}", e),
    };
}

async fn handle_list_command(sc: &clap::ArgMatches<'_>) {
    // Parse input
    let user = sc.value_of("user").map(|x| x.to_string()).take();
    let since = sc
        .value_of("since")
        .map(|x| DateTime::parse_from_rfc3339(x).unwrap())
        .take();
    // Process input
    let gists = gstm::list(user, since).await;
    // Show output
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

                let username: String = g.owner.map_or(String::new(), |o| o.login.unwrap());
                println!(
                    "{} {} {} {}",
                    g.created_at.unwrap(),
                    username,
                    g.id.unwrap(),
                    description
                );
            }
        }
        Err(e) => log::error!("Error retrieving git listing: {}", e),
    }
}

async fn handle_get_command(sc: &ArgMatches<'_>) {
    let id = String::from(sc.value_of("id").unwrap());
    let fs_dest = sc.value_of("output");
    let is_greedy = sc.is_present("greedy") || fs_dest.is_some();
    let no_content = sc.is_present("no-content");
    let delimiter = sc.value_of("delimiter").unwrap_or("\n");

    let gist = gstm::get(id.clone()).await;
    if let Err(e) = gist {
        log::error!("Failed to get {}: {}", id, e);
        return;
    }
    let mut files = gist.unwrap().files;

    if !no_content && is_greedy {
        log::debug!("Truncation requirement recognised. Iterating for truncation");
        let client = reqwest::Client::new();
        for (filename, file) in files.iter_mut() {
            if file.truncated.unwrap() {
                let endpoint = file.raw_url.as_ref().unwrap();
                log::debug!("{} truncated, downloading from {}", filename, endpoint);
                let req = client.get(endpoint.as_str()).header("user-agent", "gstm");
                let res = req.send().await.unwrap();
                let content = res.text().await;
                file.content = content.ok();
                log::debug!("Contents updated for {}", filename);
            }
        }
    }

    for (filename, file) in files.iter() {
        let header = format!(
            "{} {} {}",
            filename,
            file.language.as_ref().unwrap(),
            file.size
        );
        let body = match no_content {
            false => file.content.as_ref().unwrap(),
            true => "",
        };
        let output = format!("{}\n{}{}", header, body, delimiter);
        match fs_dest {
            Some(dst) => {
                let filepath = format!("{}/{}", dst, filename);
                log::debug!("Creating {}", filepath);
                let mut f = File::create(&filepath).unwrap();
                log::debug!("Writing to {}", filepath);
                match f.write_all(file.content.as_ref().unwrap().as_bytes()) {
                    Ok(_) => log::info!("{} written", filepath),
                    Err(e) => log::error!("Could not write to {}: {}", filepath, e),
                }
            }
            None => println!("{}", output),
        }
    }
}
