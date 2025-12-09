use std::{env, path::Path, process};

use git_helpers::get_local_hash;

pub mod cli_definitions;
pub mod command_manager;
pub mod config;
pub mod errors;
pub mod git_helpers;

pub fn get_default_path() -> String {
    let ref home_dir = env::var("HOME").unwrap_or("".to_string());
    let file_name = String::from(".git-favorite-branch-config");
    let path_raw: String;
    let default_path = match home_dir.len().cmp(&0) {
        std::cmp::Ordering::Greater => {
            path_raw = format!("{}/{}", home_dir, file_name).to_string();
            Path::new(&path_raw)
        }
        _ => {
            path_raw = format!("./{}", file_name).to_string();
            Path::new(&path_raw)
        }
    };
    String::from(default_path.to_str().unwrap())
}

pub fn get_build_hash() -> String {
    match option_env!("GITHUB_SHA") {
        Some(github_hash) => String::from(github_hash),
        None => match get_local_hash() {
            Ok(output) => String::from_utf8(output.stdout).unwrap_or_else(|err| {
                eprintln!("Error parsing commit hash: {}", err);
                process::exit(1);
            }),
            Err(err) => {
                eprintln!("Could not get local hash: {}", err);
                String::from("")
            }
        },
    }
}
