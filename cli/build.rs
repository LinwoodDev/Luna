use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Define the source path
    let source_path = project_root.join("node_modules/@linwooddev/style/css/main.css");

    // Define the destination path in the output directory
    let destination_path = out_dir.join("main.css");

    // Copy the file to the output directory
    fs::copy(&source_path, &destination_path)?;

    // We still want to copy to the public directory for rust_embed
    let public_destination_path = project_root.join("assets/public/main.css");
    fs::create_dir_all(public_destination_path.parent().unwrap())?;
    fs::copy(&source_path, &public_destination_path)?;

    println!("cargo:rerun-if-changed={}", source_path.display());
    println!("cargo:rerun-if-changed=public");

    Ok(())
}