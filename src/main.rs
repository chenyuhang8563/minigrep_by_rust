use std::env;
use std::process;
use minigrep::Config;

/// 处理输入并保存为args迭代器
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application failed: {e}");
        process::exit(1);
    }
}