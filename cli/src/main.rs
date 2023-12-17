mod directory;
mod docs;
mod generator;

use clap::{Args, Parser, Subcommand};
use luna_api::models::RepositoryData;
use std::fs::File;
use std::io::Write;

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
        path: Option<String>,
    },
    Docs {
        /// The path where the docs should get generated. Default to: "docs"
        path: Option<String>,
        /// The path of the index file. Default to: "index.json"
        index: Option<String>,
    },
}

#[derive(Args)]
struct InspectArgs {
    #[command(subcommand)]
    command: InspectCommands,
}

#[derive(Subcommand)]
enum InspectCommands {
    Author { name: String },
    Authors,
    Asset { name: String },
    Assets,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate { path } => generate(path.to_owned()),
        Commands::Docs { path, index } => docs(path.to_owned(), index.to_owned()),
        _ => {}
    }
}

fn docs(path: Option<String>, index: Option<String>) {
    let index = index.unwrap_or(String::from("index.json"));
    let data = RepositoryData::from_index(
        std::fs::read_to_string(&index)
            .expect("Could not read index file")
            .as_ref(),
    )
    .expect("Could not parse index file");
    match { docs::generate_docs(&data, path.unwrap_or(".".to_string())) } {
        Ok(_) => {
            println!("Successfully generated docs.");
        }
        Err(error) => {
            eprintln!("Error while generating docs: {}", error);
        }
    }
}

fn generate(path: Option<String>) {
    let path = path.unwrap_or(String::from("index.json"));
    let directory = directory::RepositoryDirectory::new(None);
    match { directory.generate_index() } {
        Ok(data) => {
            let mut file = File::create(&path).expect("Cannot create file");
            file.write_all(data.to_index().expect("Could not generate json").as_ref())
                .expect("Could not write file");
            println!("Successfully generated index file at {:?}.", path);
        }
        Err(error) => {
            eprintln!("Error while generating index: {}", error);
        }
    }
}
