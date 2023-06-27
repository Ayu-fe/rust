// mod a {
//   pub mod b {
//     pub mod d {
//       pub fn add() {}
//     }
//   }
//   mod c {
//     fn multi() {}
//   }
// }

// // 使用use关键字
// // 重导是什么？
// pub use crate::a::b::d;

// fn main() {
//   println!("123");
// }

use std::fs;
use std::error::Error;
use std::env;

// 封装配置结构体  已new的方式构造配置 返回Result处理错误
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub insensitive: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("error");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        // 读取INSENSITIVE 这个环境变量 is_err() 返回一个bool
        let insensitive = env::var("INSENSITIVE").is_err();
        Ok(Config { query, file_name, insensitive })
    }
}

// 保持main函数简洁 抽离关键逻辑 返回Result处理错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let target_str = fs::read_to_string(config.file_name)?;
    
    // 控制流  这种写法还是头一次见  不得不说非常方便
    let result = if config.insensitive {
        search_with_insensitivie(&config.query, &target_str)
    } else {
        search(&config.query, &target_str)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_with_insensitivie<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

// 测试
#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test_search() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// struct Rect {
//     width: u32,
//     height: u32
// }

// impl Rect {
//     fn can_hold(&self, other: &Rect) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_can_hold() {
//         let a = Rect {
//             width: 12,
//             height: 10
//         };
//         let b = Rect {
//             width: 10,
//             height: 5
//         };
//         assert!(a.can_hold(&b))
//     }

//     #[test]
//     fn internal() {
//         assert_eq!(4, internal_adder(2, 2));
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     internal_adder(a, 2)
// }

// fn internal_adder(a: i32, b: i32) -> i32 {
//     a + b
// }
