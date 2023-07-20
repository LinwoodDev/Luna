mod directory;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Index(InspectArgs),
    Get(InspectArgs),
    Generate { },
}

#[derive(Args)]
struct InspectArgs {
    #[command(subcommand)]
    command : InspectCommands,
}

#[derive(Subcommand)]
enum InspectCommands {
    Author {
        name: String,
    },
    Authors,
    Asset {
        name: String,
    },
    Assets,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate {} => {
            println!("Generating...");
        }
        _ => {}
    }
}