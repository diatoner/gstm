use directories::ProjectDirs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{prelude::*, stdin, stdout};
use std::path::PathBuf;

pub fn get_token_cache_path() -> PathBuf {
    let project_folder = ProjectDirs::from("", "", "gstm").unwrap();
    let cache_folder = project_folder.cache_dir();
    cache_folder.join("token")
}

pub fn get_cached_token() -> Option<String> {
    let token_cache_path = get_token_cache_path();
    log::debug!("Auth token cache path retrieved: {:?}", token_cache_path);
    File::open(token_cache_path).map_or_else(
        |e| {
            log::error!("Failed to read cached auth token: {}", e);
            None
        },
        |mut v| {
            let mut token = String::new();
            v.read_to_string(&mut token).unwrap();
            log::debug!("Cached token retrieved");
            Some(token.trim().to_string())
        },
    )
}

pub fn get_new_token() -> Option<String> {
    println!("GitHub access token required.\nPlease open https://github.com/settings/tokens/new and create one with 'gist' permissions, then enter it here.");
    print!("Token: ");
    stdout().flush().unwrap();
    let mut token = String::new();
    match stdin().read_line(&mut token) {
        Ok(_) => {
            token = token.trim().to_string();

            // Ensure cache directories exist
            let cache_path = get_token_cache_path();
            if let Some(cache_dir) = cache_path.into_boxed_path().parent() {
                create_dir_all(cache_dir).unwrap();
            }

            // Write to cache
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(get_token_cache_path())
                .map_or_else(
                    |e| {
                        log::error!("Failed to open token cache path: {}", e);
                        None
                    },
                    |mut v| {
                        let token_bytes = token.as_bytes();
                        if let Err(e) = v.write_all(token_bytes) {
                            log::error!("Failed to write token to cache: {}", e);
                        };
                        Some(token)
                    },
                )
        }
        Err(e) => {
            log::error!("Unkown error reading from stdin");
            log::error!("{:?}", e);
            None
        }
    }
}
