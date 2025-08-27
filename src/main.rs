mod cmd;
mod types;
mod utils;

use clap::Parser;
use cmd::{clean::clean, list::list, remove::remove, save::save, tp::teleport};
use utils::get_current_dir;

#[derive(Parser, Debug)]
#[command(author = "Eli Front", version = "0.1", about = "Teleport", long_about = None)]
struct Cli {
    #[arg(help = "Enter a directory name to navigate to or use the 'save' subcommand")]
    input: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    #[command(about = "Saves the current directory")]
    Save {
        #[arg(help = "Enter a name for the directory", short, long)]
        name: Option<String>,
    },
    #[command(about = "Lists all saved directories", name = "ls")]
    List,
    #[command(about = "Delete all saved directories", name = "clean")]
    Clean,
    #[command(about = "Delete a single directory by name", name = "rm")]
    Remove {
        #[arg(help = "Enter a name for the directory", short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // get current directory
    let current_dir = get_current_dir();

    match &cli.command {
        Some(Commands::Save { name }) => save(&current_dir, name),
        Some(Commands::List) => list(),
        Some(Commands::Clean) => clean(),
        Some(Commands::Remove { name }) => remove(name),
        None => {
            teleport(&cli);
        }
    }
}
