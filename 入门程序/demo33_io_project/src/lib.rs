use std::fs;
use std::env;
use std::error::Error;

//定义一个结构体，用于保存参数值
pub struct Config {
    pub query: String,  //query 字段用于保存要搜索的字符串
    pub file_path: String,  //file_path 字段用于保存要搜索的文件名
    pub ignore_case: bool,  //ignore_case 字段用于保存是否忽略大小写
}

//将 parse_config 变为 Config::new
impl Config {
    pub fn build(/*args: &[String]*/
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        /* //检查参数个数是否正确
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok(); //IGNORE_CASE 环墋变量存在时，env::var 函数会返回一个 Result，其值是 Ok。否则，它会返回一个包含环境变量值的 Err */

        args.next();    //跳过第一个参数，因为它是程序名

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

//提取 run 函数来包含剩余的程序逻辑
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型
    let contents = fs::read_to_string(config.file_path)?; //? 运算符，它可以将 Result 的返回值传递给 Ok 的值，如果是 Err，则将 Err 的值返回给调用者
    
    //根据 config.ignore_case 的值调用 search 或 search_case_insensitive
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

//创建一个我们期望的 search 函数的测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitsive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

//该函数接收一个查询字符串和一个文本字符串，并返回文本字符串中包含查询字符串的所有行
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { //告诉 Rust 函数 search 返回的数据将与 search 函数中的参数 contents 的数据存在的一样久
    /* let mut results = Vec::new();   //存储匹配的行

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//实现 search_case_insensitive 函数
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();   //将 query 转换为小写
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {   //将 line 转换为小写并检查是否包含 query
            results.push(line);
        }
    }

    results
}