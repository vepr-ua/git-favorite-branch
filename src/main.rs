use clap::Parser;
use gfb::{
    cli_definitions::{Cli, Commands},
    command_manager::CommandManager,
    config::Config,
    errors::Result,
    get_default_path,
    git::{contract::GitProvider, gitlib2},
};
use std::collections::HashMap;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let default_path = get_default_path();
    let mut config = Config::new(&default_path, HashMap::new());
    config.load()?;

    // let git_provider = GitShell {};
    let git_provider = gitlib2::GitLib::new_with_repo(None)?;
    let active_branch = git_provider.get_current_branch()?;

    let mut cmd_manager = CommandManager {
        config,
        git: Box::new(git_provider),
    };

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { key, branch_name } => {
            let branch = match branch_name {
                Some(v) => v.trim().to_owned(),
                None => active_branch,
            };

            cmd_manager.add_branch(key.trim().to_owned(), branch)?;
        }
        Commands::Use { key } => cmd_manager.switch_to_branch(key.trim().to_owned())?,
        Commands::Del { key } => cmd_manager.delete_branch(key.trim().to_owned())?,
        Commands::DelAll => cmd_manager.clear_branches()?,
        Commands::Branch { key } => match key {
            Some(v) => {
                let branch_name = cmd_manager.print_branch_name(v.trim().to_owned())?;
                println!("{}", branch_name);
            }
            None => {
                println!("{}", active_branch);
            }
        },
        Commands::New { key } => cmd_manager.create_new_branch(key.trim().to_owned())?,
        Commands::Ls => match cmd_manager.list_branches() {
            Some(branches) => {
                for (k, v) in branches {
                    println!("{} -> {}", k, v);
                }
            }
            None => println!("No favorite branches saved"),
        },
        Commands::Rename { keys } => {
            let (input_new_key, input_old_key) = match keys.as_slice() {
                [new_key] => (new_key.as_str().trim(), None),
                [old_key, new_key] => (new_key.as_str().trim(), Some(old_key.as_str().trim())),
                _ => unreachable!("Command requires either new key or both new and old keys"),
            };

            let (old_key, new_key) = cmd_manager.rename_key(input_new_key, input_old_key)?;
            println!("Renamed key from '{}' to '{}'", old_key, new_key);
        }
        Commands::Sync { key } => {
            let key = key.trim();
            cmd_manager.sync_branch(key)?;
        }
    };

    Ok(())
}
