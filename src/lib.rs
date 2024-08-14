use std::{env, error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub target: String,
    pub is_senistive: bool,
}
impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("the args is not bad");
        }
        Ok(Config {
            file_path: args[2].clone(),
            target: args[1].clone(),
            is_senistive: env::var("INGORE_CASE").is_err(),
        })
    }
}

pub fn read_file_content(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    println!("---{}", config.is_senistive);

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
    let mut ret = Vec::new();
    for line in str.lines() {
        if line.contains(target) {
            ret.push(line);
        }
    }
    ret
}
fn ingore_search<'a>(str: &'a String, target: &str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let target = target.to_lowercase();
    for line in str.lines() {
        if line.to_lowercase().contains(&target) {
            ret.push(line);
        }
    }
    ret
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
