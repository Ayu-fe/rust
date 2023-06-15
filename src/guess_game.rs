/*
 猜数字游戏
*/
// use std::str::FromStr;
// use std::env;
use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};
// 导入io 和io::Write
// use std::io::{self, Write};

fn main() {
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("随机生成的数是{}", secret_number);

    loop {
        println!("输入一个字符");
        let mut gusses = String::new();

        // 处理用户输入
        io::stdin()
            .read_line(&mut gusses)
            .expect("something is wrong");
        // println!("你的输入是 {}", gusses);

        // 从崩溃报错到处理错误
        let gusses: u32 = match gusses.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 匹配结果
        match gusses.cmp(&secret_number) {
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
            Ordering::Greater => println!("太大了"),
            Ordering::Less => println!("太小了"),
        }
    }
}

pub fn start() {
    main()
}
