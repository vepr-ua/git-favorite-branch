use std::{env, path::Path};

pub mod cli_definitions;
pub mod command_manager;
pub mod config;
pub mod errors;
pub mod git;

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
