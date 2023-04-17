use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

pub fn get_vault_dir() -> PathBuf {
    dotenv().ok();
    Path::new(&env::var("VAULT_DIR").unwrap_or_default()).to_path_buf()
}
