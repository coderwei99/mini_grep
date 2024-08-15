use std::{env, error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub target: String,
    pub is_senistive: bool,
}
impl Config {
    pub fn parse_args(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("the args is not bad");
        }
        args.next();
        let target = match args.next() {
            Some(target) => target,
            None => return Err("params is error"),
        };
        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("params is error"),
        };
        Ok(Config {
            file_path,
            target,
            is_senistive: env::var("INGORE_CASE").is_err(),
        })
    }
}

pub fn read_file_content(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    println!("---{}", config.target);
    let content = fs::read_to_string(&config.file_path)?;

    let ret = if config.is_senistive {
        search(&content, &config.target)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        ingore_search(&content, &config.target)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    };
    Ok(ret)
}

fn search<'a>(str: &'a String, target: &str) -> Vec<&'a str> {
    // let mut ret = Vec::new();
    // for line in str.lines() {
    //     if line.contains(target) {
    //         ret.push(line);
    //     }
    // }
    // ret
    str.lines().filter(|str| str.contains(target)).collect()
}
fn ingore_search<'a>(str: &'a String, target: &str) -> Vec<&'a str> {
    str.lines()
        .filter(|str| str.to_lowercase().contains(&target.to_lowercase()))
        .collect()
}

#[test]
fn case_search() {
    let str = String::from(
        "\
hello world.
this is my first rust project.
    ",
    );
    let ret = search(&str, "hello");
    assert_eq!(vec!["hello world."], ret);
}

#[test]
fn ingore_case_search() {
    let str = String::from(
        "\
Hello world.
this is my first rust project.
hello coderwei!
    ",
    );

    assert_eq!(
        vec!["Hello world.", "hello coderwei!"],
        ingore_search(&str, "hello")
    )
}
