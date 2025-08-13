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
    /// Inspect the generated index file
    Index(InspectArgs),
    /// Inspect the current repository
    Get(InspectArgs),
    /// Generate an index file out of the current repository
    Generate {
        /// The path where the index file should get generated.
        #[arg(default_value = "output/index.json")]
        path: String,

    },
    /// Generate documentation for the current index file
    Docs(DocsArgs),
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

#[derive(Args)]
struct DocsArgs {
    /// The path where the docs should get generated.
    #[arg(default_value = "output/docs")]
    path: String,
    /// The path of the index file.
    #[arg(default_value = "output/index.json")]
    index: String,
    /// The page size of lists. Default to: 20
    #[arg(long, default_value_t = 20)]
    page_size: usize,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate { path } => generate(path.to_owned()),
        Commands::Docs(args) => docs(args.path.to_owned(), args.index.to_owned(), args.page_size),
        _ => {}
    }
}

fn docs(path: String, index: String, page_size: usize) {
    let data = RepositoryData::from_index(
        std::fs::read_to_string(&index)
            .unwrap_or_else(|_| panic!("Could not read index file {index}"))
            .as_str(),
    )
    .expect("Could not parse index file");
    let result = docs::generate_docs(&data, path, page_size);
    match result {
        Ok(_) => {
            println!("Successfully generated docs.");
        }
        Err(error) => {
            eprintln!("Error while generating docs: {error}");
        }
    }
}

fn generate(path: String) {
    let path = std::path::PathBuf::from(path);
    std::fs::create_dir_all(path.parent().unwrap_or(&path)).expect("Could not create directory");
    let directory = directory::RepositoryDirectory::new(None);
    let result = directory.generate_index();
    match result {
        Ok(data) => {
            let mut file = File::create(&path).expect("Cannot create file");
            file.write_all(data.to_index().expect("Could not generate json").as_ref())
                .expect("Could not write file");
            println!("Successfully generated index file at {path:?}.");
        }
        Err(error) => {
            eprintln!("Error while generating index: {error}");
        }
    }
}
