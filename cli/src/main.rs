mod directory;
mod docs;
mod generator;

use anyhow::{Context, Result};
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
    Index {
        #[command(flatten)]
        args: InspectArgs,
        /// The path of the index file.
        #[arg(default_value = "output/index.json")]
        path: String,
    },
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
    /// Generate an index file and documentation
    Build(DocsArgs),
}

#[derive(Args, Clone)]
struct InspectArgs {
    #[command(subcommand)]
    command: InspectCommands,
}

#[derive(Subcommand, Clone)]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate { path } => generate(path.to_owned())?,
        Commands::Docs(args) => docs(args.path.to_owned(), args.index.to_owned(), args.page_size)?,
        Commands::Build(args) => {
            generate(args.index.to_owned())?;
            docs(args.path.to_owned(), args.index.to_owned(), args.page_size)?;
        }
        Commands::Index { args, path } => {
            let index_content = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read index file {path}"))?;

            let data = RepositoryData::from_index(&index_content)
                .context("Could not parse index file")?;

            inspect(&data, args)?;
        }
        Commands::Get(args) => {
            let directory = directory::RepositoryDirectory::default();
            let data = directory.generate_index()
                .context("Error while generating index")?;

            inspect(&data, args)?;
        }
    }
    Ok(())
}

fn inspect(data: &RepositoryData, args: &InspectArgs) -> Result<()> {
    match &args.command {
        InspectCommands::Author { name } => {
            let author = data.authors.iter().find(|a| a.name == *name);
            if let Some(author) = author {
                println!("{:#?}", author);
            } else {
                println!("Author not found");
            }
        }
        InspectCommands::Authors => {
            for author in &data.authors {
                println!("{}", author.name);
            }
        }
        InspectCommands::Asset { name } => {
            let asset = data.assets.iter().find(|a| a.name == *name);
            if let Some(asset) = asset {
                println!("{:#?}", asset);
            } else {
                println!("Asset not found");
            }
        }
        InspectCommands::Assets => {
            for asset in &data.assets {
                println!("{}", asset.name);
            }
        }
    }
    Ok(())
}

fn docs(path: String, index: String, page_size: usize) -> Result<()> {
    let index_content = std::fs::read_to_string(&index)
        .with_context(|| format!("Could not read index file {index}"))?;

    let data = RepositoryData::from_index(&index_content)
        .context("Could not parse index file")?;

    docs::generate_docs(&data, path, page_size)
        .context("Error while generating docs")?;

    println!("Successfully generated docs.");
    Ok(())
}

fn generate(path: String) -> Result<()> {
    let path = std::path::PathBuf::from(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Could not create directory")?;
    }

    let directory = directory::RepositoryDirectory::default();
    let data = directory.generate_index()
        .context("Error while generating index")?;

    let mut file = File::create(&path).context("Cannot create file")?;
    let json = data.to_index().context("Could not generate json")?;
    file.write_all(json.as_ref())
        .context("Could not write file")?;

    println!("Successfully generated index file at {path:?}.");
    Ok(())
}
