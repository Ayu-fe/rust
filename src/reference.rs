use std::io;

fn main() {
    // let a = 2.3;
    // let b = 1.1;
    // println!("result is {}", a / b);
    // let arr = [1, 2, 3];

    // let mut idx = String::new();
    // io::stdin()
    //     .read_line(&mut idx)
    //     .expect("error");

    // let idx: usize = idx.trim().parse().expect("error");

    // let res = arr[idx];
    // println!("your input is {}", res);

    // let num = if true {5} else {6};

    // let tmp = 1;
    // 'label: loop {
    //     if tmp == 2 {
    //         break 'label;
    //     }
    // }

    // fiber
    let res: u32 = fiber(6);
    println!("res: {}", res);

    let mut str: String = String::from("hello");
    str.push_str("world");

    let mut tStr: &str = "123";

    let len = getStrLen(&mut str);
    // 在同一时间，只能有一个对某一特定数据的可变引用
    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    // 引用被返回并不会保证变量不会释放 实际上 当前作用域创建的变量会在作用域结束后释放 rust不允许返回该变量的指针 否则会产生一个悬垂指针
}

// 0 1 1 2 3 5
fn fiber(mut n: u32) -> u32 {
    if n == 1 {
        return 0;
    }
    if n == 2 || n == 3 {
        return 1;
    }
    let mut first = 1;
    let mut second = 1;
    let mut res = 0;
    while n > 3 {
        res = first + second;
        first = second;
        second = res;
        n -= 1;
    }
    res
}

fn test(f: fn() -> fn()) -> fn() {
    f()
}

fn getStrLen(s: &mut String) -> usize {
    s.len()
}

pub fn start() {
    main()
}
