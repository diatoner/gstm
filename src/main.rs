use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use gst;

#[tokio::main]
async fn main() {
    let matches = App::new("gst")
        .author(crate_authors!())
        .version(crate_version!())
        .about("GitHub Gist manipulator")
        .subcommand(
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
        )
        .get_matches();

    match matches.subcommand() {
        ("create", Some(sc)) => {
            let files: Vec<String> = sc.values_of("files").unwrap().map(String::from).collect();
            let is_public = sc.is_present("private");
            let description = match sc.value_of("description") {
                Some(s) => Some(String::from(s)),
                None => None,
            };
            let res = gst::create(files, is_public, description).await;
            match res {
                Ok(value) => println!("Gist available at {}", value.html_url),
                Err(e) => println!("An error occurred:\n\t{:?}", e),
            };
        }
        _ => {}
    }
}
