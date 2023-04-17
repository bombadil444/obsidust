use clap::Parser;
use std::fs::{create_dir, rename};
use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;
mod utils;

/// Command line tools for Obsidian
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// prefix of files to move
    #[arg(short, long)]
    prefix: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let vault_dir: PathBuf = utils::get_vault_dir();

    move_files(&vault_dir, &args.prefix)?;

    Ok(())
}

// move files that start with a given prefix to a folder with the same name as the prefix
fn move_files(vault_dir: &PathBuf, prefix: &str) -> Result<()> {
    let dest_dir: PathBuf = vault_dir.join(prefix);

    if !dest_dir.exists() {
        create_dir(&dest_dir)?;
    }

    let mut count: u32 = 0;

    for entry in WalkDir::new(vault_dir).max_depth(1).into_iter().flatten() {
        if entry.file_type().is_file() // avoid matching on folders
            && entry.file_name().to_str().unwrap().starts_with(prefix)
        {
            let dest_path: PathBuf = dest_dir.join(entry.file_name());
            rename(entry.path(), dest_path)?;
            count += 1;
        }
    }

    println!("{} files moved to: {}", count, dest_dir.display());

    Ok(())
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
