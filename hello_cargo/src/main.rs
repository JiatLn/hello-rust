use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!(
        "fileName is {}, query is {}.",
        config.file_name, config.query
    );

    let content = fs::read_to_string(config.file_name).expect("文件读取错误");
    println!("The content is:\n{}", content);
}

struct Config {
    file_name: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let file_name = args[1].clone();
        let query = args[2].clone();

        Config { file_name, query }
    }
}
