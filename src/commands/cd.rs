use colored::*;
use std::env;
use std::path::Path;

pub fn change_directory<'a>(mut args: impl Iterator<Item = &'a str>) {
    let new_dir = args.next().unwrap_or("/");
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", format!("Failed to change directory: {}", e).red());
    }
}
