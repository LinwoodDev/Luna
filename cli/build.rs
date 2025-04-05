use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?); // Use CARGO_MANIFEST_DIR

    println!("cargo:warning=Project Root: {}", project_root.display());

    let source_path = project_root.join("node_modules/@linwooddev/style/css/main.css");
    println!("cargo:warning=Source Path: {}", source_path.display());

    let destination_path = project_root.join("assets/public/main.css");

    fs::create_dir_all(destination_path.parent().unwrap())?;
    fs::copy(&source_path, &destination_path)?;

    println!("cargo:rerun-if-changed={}", source_path.display());
    println!("cargo:rerun-if-changed=public");

    Ok(())
}