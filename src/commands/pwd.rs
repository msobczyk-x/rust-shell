use std::env;

pub fn get_current_directory() -> String {
    env::current_dir()
        .map(|path| {
            let path_str = path.display().to_string();
            let home_dir = format!("/Users/{}", env::var("USER").unwrap_or_default());

            // Replace the home directory path with '~' if applicable
            if path_str.starts_with(&home_dir) {
                path_str.replace(&home_dir, "~")
            } else {
                path_str
            }
        })
        .unwrap_or_else(|_| String::from("Unknown Directory"))
}
