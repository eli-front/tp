use clap::CommandFactory;

use crate::{utils::get_saved_dirs, Cli};

pub fn teleport(cli: &Cli) {
    let dirs = get_saved_dirs();

    match &cli.input {
        Some(input) => {
            if let Some(dir) = dirs.iter().find(|d| d.name == *input) {
                println!("tp::cd {}", dir.path);
            } else {
                println!("'{}' is not a recognized directory name.", input);

                if !dirs.is_empty() {
                    println!("Available directories are:");
                    for dir in dirs {
                        println!(" - {}", dir.name);
                    }
                }

                // Optionally, show help message
                Cli::command().print_help().unwrap();
                println!();
            }
        }
        None => {
            // No input given, just print help
            Cli::command().print_help().unwrap();
            println!();
        }
    }
}
