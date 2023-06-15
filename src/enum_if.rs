mod back_of_house;

// use back_of_house::host;

// 枚举&控制流
#[derive(Debug)]
enum NetType {
    // V3,
    // V4,
    // V5 = 8,
    V6(u8, u8, u8, u8),
    str(String),
}

enum Coin {
    First,
    Second,
    Third
}

impl NetType {
    fn call(&self) {}
}

// struct IP {
//   netType: NetType,
//   address: String
// }

fn main() {
    // let home = IP {
    //   netType: NetType::V3,
    //   address: String::from("127.0.0.1")
    // };
    let v6 = NetType::V6(127, 0, 0, 0);
    // let (a, b, c, d) = v6;
    println!("v6: {:#?}", v6);
    let str = NetType::str(String::from("new str"));
    println!("str: {:#?}", str);
    str.call();
    // let (s) = str;
    // println!("s: {:#?}", s);

    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;

    // Option 类型
    /* enum Option<T> {
         Some(T),
         None,
     }
    */
    let some = Some(123);
    let none: Option<i32> = None;

    let coin = Coin::Second;
    choose_coin(coin);

    let one = Some(1);
    let two = plus_one(one);

    let third = Some(2);
    if third == Some(2) {
        println!("ok")
    }
    print!("123");

    back_of_house::eat_breakfast();
    let a = back_of_house::host::Breakfast::choose_breakfase("toast");
}

fn choose_coin(coin: Coin) {
    match coin {
        Coin::First => 1,
        Coin::Second => {
            println!("this is {}", 2);
            2
        },
        Coin::Third => 3
    };
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    // match i {
    //     Some(t) => Some(t + 1),
    //     _ => Some(12)
    // }
    if let Some(2) = i { 
        return Some(3)
    }
    Some(2)
}

pub fn start() {
    main()
}