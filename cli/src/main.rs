mod create;
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
    /// Create new files
    #[command(subcommand)]
    Create(CreateCommands),
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
    /// Preview the generated documentation
    Preview(PreviewArgs),
}

#[derive(Subcommand)]
enum CreateCommands {
    /// Create a new repository configuration file (config.toml)
    Repository(CreateRepositoryArgs),
    /// Create a new author file (content/{author}/author.toml)
    Author(CreateAuthorArgs),
    /// Create a new asset file (content/{author}/{name}/asset.toml)
    Asset(CreateAssetArgs),
}

#[derive(Args)]
struct CreateRepositoryArgs {
    /// The name of the repository. Defaults to the name of the created directory.
    #[arg(short, long)]
    name: Option<String>,
    /// The description of the repository.
    #[arg(short, long)]
    description: Option<String>,
    /// The path of the repository
    #[arg(short, long, default_value = ".")]
    path: String,
    /// Overwrite the file if it already exists.
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct CreateAuthorArgs {
    /// The name of the author.
    #[arg()]
    name: String,
    // The display name of the author.
    #[arg(short, long)]
    display_name: Option<String>,
    /// THe avatar URL of the author.
    #[arg(short, long)]
    avatar_url: Option<String>,
    /// The description of the author.
    #[arg(short, long)]
    description: Option<String>,
    /// The email of the author.
    #[arg(short, long)]
    email: Option<String>,
    /// Links related to the author.
    #[arg(short, long)]
    links: Vec<String>,
    /// The path of the repository
    #[arg(short, long, default_value = ".")]
    path: String,
    /// Overwrite the file if it already exists.
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct CreateAssetArgs {
    /// The name of the asset.
    #[arg()]
    name: String,
    /// The description of the asset.
    #[arg(short, long)]
    description: Option<String>,
    /// The path of the repository
    #[arg(short, long, default_value = ".")]
    path: String,
    /// Overwrite the file if it already exists.
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct PreviewArgs {
    /// The path where the docs are generated.
    #[arg(default_value = "output/docs")]
    path: String,
    /// The port to serve on.
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
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
    /// The strategy for bundling assets.
    #[arg(long, default_value_t = docs::BundleStrategy::Images, value_enum)]
    bundle: docs::BundleStrategy,
    /// Optional root directory for custom docs templates and static files.
    ///
    /// Expected structure:
    /// - templates/**/*.hbs
    /// - components/**/*.hbs
    /// - layouts/**/*.hbs
    /// - public/**/*
    #[arg(long)]
    custom_root: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create(create_command) => create::create(create_command)?,
        Commands::Generate { path } => generate(path.to_owned())?,
        Commands::Docs(args) => docs(
            args.path.to_owned(),
            args.index.to_owned(),
            args.page_size,
            args.bundle.clone(),
            args.custom_root.to_owned(),
        )?,
        Commands::Build(args) => {
            generate(args.index.to_owned())?;
            docs(
                args.path.to_owned(),
                args.index.to_owned(),
                args.page_size,
                args.bundle.clone(),
                args.custom_root.to_owned(),
            )?;
        }
        Commands::Preview(args) => preview(args.path.to_owned(), args.port)?,
        Commands::Index { args, path } => {
            let index_content = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read index file {path}"))?;

            let data =
                RepositoryData::from_index(&index_content).context("Could not parse index file")?;

            inspect(&data, args)?;
        }
        Commands::Get(args) => {
            let directory = directory::RepositoryDirectory::default();
            let data = directory
                .generate_index()
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

fn docs(
    path: String,
    index: String,
    page_size: usize,
    bundle: docs::BundleStrategy,
    custom_root: Option<String>,
) -> Result<()> {
    let index_content = std::fs::read_to_string(&index)
        .with_context(|| format!("Could not read index file {index}"))?;

    let data = RepositoryData::from_index(&index_content).context("Could not parse index file")?;

    docs::generate_docs(&data, path, page_size, bundle, custom_root)
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
    let data = directory
        .generate_index()
        .context("Error while generating index")?;

    let mut file = File::create(&path).context("Cannot create file")?;
    let json = data.to_index().context("Could not generate json")?;
    file.write_all(json.as_ref())
        .context("Could not write file")?;

    println!("Successfully generated index file at {path:?}.");
    Ok(())
}

fn preview(path: String, port: u16) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let app = axum::Router::new().fallback_service(tower_http::services::ServeDir::new(path));
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
        println!("Listening on http://localhost:{}", port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}
