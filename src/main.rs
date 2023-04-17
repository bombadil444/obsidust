use clap::Parser;
use std::fs::{create_dir, rename};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

/// Command line tools for Obsidian
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// prefix of files to move
    #[arg(short, long)]
    prefix: String,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // TODO: make dynamic
    let user_path: &str = "/home/bombadil";
    let notes_path: &str = "vault/daily notes";

    move_files(user_path, notes_path, &args.prefix)?;

    Ok(())
}

fn move_files(user_path: &str, notes_path: &str, prefix: &str) -> io::Result<()> {
    let vault_dir: PathBuf = Path::new(user_path).join(notes_path);
    let dest_dir: PathBuf = vault_dir.join(prefix);

    if !dest_dir.exists() {
        create_dir(&dest_dir)?;
    }

    let mut count: u32 = 0;

    for entry in WalkDir::new(&vault_dir).max_depth(1).into_iter().flatten() {
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
