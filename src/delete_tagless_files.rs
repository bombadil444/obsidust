use regex::Regex;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
mod utils;

fn main() -> Result<()> {
    let vault_dir: PathBuf = utils::get_vault_dir();

    delete_tagless_files(&vault_dir)?;

    Ok(())
}

// locate files without tags and delete them
fn delete_tagless_files(vault_dir: &PathBuf) -> Result<()> {
    let mut filelist: Vec<String> = Vec::new();

    // locate files without tags
    for entry in WalkDir::new(vault_dir).into_iter().flatten() {
        if entry.file_type().is_file() {
            let file = File::open(entry.path()).unwrap();
            let reader = BufReader::new(file);

            let mut found_hashtag: bool = false;

            for line in reader.lines() {
                // if line starts with a '#' which is followed by a non-space character, it's a tag
                let re = Regex::new(r"#\S+").unwrap();

                if re.is_match(&line.unwrap()) {
                    found_hashtag = true;
                    break;
                }
            }

            if !found_hashtag {
                filelist.push(entry.path().to_str().unwrap().to_owned());
            }
        }
    }

    // delete the files
    filelist.sort();
    for path in &filelist {
        println!("{}", path);
        remove_file(Path::new(path))?;
    }

    println!("{} files without tags deleted", filelist.len());

    Ok(())
}
