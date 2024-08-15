use mini_grep::Config;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let content = mini_grep::read_file_content(&config).unwrap_or_else(|err| {
        eprintln!(
            "rust perject have error, because can't read file path, it's not exist: {}",
            err
        );
        process::exit(1);
    });
    println!("{:?}", content);
}
