use std::{env, path::Path};
use std::iter::Peekable;

pub fn change_directory<'a>(args: &mut Peekable<impl Iterator<Item = &'a str>>) {
    let new_dir = args.peek().map_or("/", |x| *x); // Default to '/' if no argument
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("Failed to change directory: {}", e);
    }
}
