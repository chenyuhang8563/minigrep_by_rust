use std::{env, error::Error, fs};
/// 结构体Config
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
} 
/// 结构体实例build方法
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query Not Found"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File_path Not Found"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


/// 运行
pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
       
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else{
        search(&config.query, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}

/// 查找（不区分大小写）
pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 查找（区分大小写）
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
   