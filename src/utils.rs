use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

pub fn get_vault_dir() -> PathBuf {
    dotenv().ok();
    Path::new(&env::var("VAULT_DIR").unwrap_or_default()).to_path_buf()
}

#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
