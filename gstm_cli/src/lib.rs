use chrono::DateTime;
use clap::ArgMatches;
use std::fs::File;
use std::io::prelude::*;

mod auth;

pub async fn handle_create_command(sc: &ArgMatches<'_>) {
    let files: Vec<String> = sc.values_of("files").unwrap().map(String::from).collect();
    let is_public: bool = sc.is_present("public");
    let description: Option<String> = sc.value_of("description").map(|x| x.to_string()).take();

    log::info!("Retrieving cached auth token");
    let token = match auth::get_cached_token() {
        Some(t) => t,
        None => auth::get_new_token().unwrap_or_default(),
    };

    log::info!("Performing API request");
    let res = gstm_core::create(files, is_public, description, token).await;

    match res {
        Ok(value) => println!("Gist available at {}", value.html_url.unwrap()),
        Err(e) => log::error!("Gist creation failed: {:?}", e),
    };
}

pub async fn handle_list_command(sc: &ArgMatches<'_>) {
    // Parse input
    let user = sc.value_of("user").map(|x| x.to_string()).take();
    let since = sc
        .value_of("since")
        .map(|x| DateTime::parse_from_rfc3339(x).unwrap())
        .take();
    // Process input
    let token = auth::get_cached_token();
    let gists = gstm_core::list(user, since, token).await;
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

pub async fn handle_get_command(sc: &ArgMatches<'_>) {
    let id = String::from(sc.value_of("id").unwrap());
    let fs_dest = sc.value_of("output");
    let is_greedy = sc.is_present("greedy") || fs_dest.is_some();
    let no_content = sc.is_present("no-content");
    let delimiter = sc.value_of("delimiter").unwrap_or("\n");
    let token = auth::get_cached_token();

    let gist = gstm_core::get(id.clone(), token).await;
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
        let body = if no_content {
            ""
        } else {
            file.content.as_ref().unwrap()
        };
        let output = format!("{}\n{}{}", header, body, delimiter);
        match fs_dest {
            Some(dst) => {
                let filepath = format!("{}/{}", dst, filename);
                log::debug!("Creating {}", filepath);
                let mut f = File::create(&filepath).unwrap_or_else(|e| {
                    log::error!("Couldn't create {}, unknown error: {:?}", &filepath, e);
                    panic!(e);
                });
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

pub async fn handle_fork_command(sc: &ArgMatches<'_>) {
    let id = sc.value_of("id").map(String::from).unwrap();

    log::info!("Retrieving cached auth token");
    let token = match auth::get_cached_token() {
        Some(t) => t,
        None => auth::get_new_token().unwrap_or_default(),
    };

    log::info!("Performing API request");
    let res = gstm_core::fork(id, token).await;

    match res {
        Ok(value) => println!("Fork available at {}", value.html_url.unwrap()),
        Err(e) => log::error!("Fork failed: {:?}", e),
    };
}
