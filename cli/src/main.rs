mod directory;
mod docs;
mod generator;

use std::fs::File;
use std::io::Write;
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
    Generate {
        /// The path where the index file should get generated. Default to: "index.json"
        path : Option<String>,
    },
    Docs,
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
        Commands::Generate {path} => generate(path.to_owned()),
        _ => {}
    }
}

fn generate(path: Option<String>) {
    let path = path.unwrap_or(String::from("index.json"));
    let directory = directory::RepositoryDirectory::new(None);
    match {
        directory.generate_index()
    } {
        Ok(data) => {
            let mut file = File::create(&path).expect("Cannot create file");
            file.write_all(data.to_index().expect("Could not generate json").as_ref()).expect("Could not write file");
            println!("Successfully generated index file at {:?}.", path);
        },
        Err(error) => {
            eprintln!("Error while generating index: {}", error);
        }
    }
}
