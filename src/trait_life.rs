use std::fmt::Display;

struct Rect<T, U> {
    x: T,
    y: U,
}
impl<T, U> Rect<T, U> {
    fn mix<V, W>(self, other: Rect<V, W>) -> Rect<T, W> {
        Rect {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summery {
    fn summarize(&self) -> String;
}

struct Str {
    x: String,
    y: String,
}

impl Summery for Str {
    fn summarize(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

fn main() {
    let numRect = Rect { x: 1, y: true };

    let strRect = Rect { x: '1', y: false };
    let res: Rect<i32, bool> = numRect.mix(strRect);
    println!("result is {} {}", res.x, res.y);

    let cus_str = Str {
        x: String::from("a"),
        y: String::from("b"),
    };

    println!("str is {}", cus_str.summarize());
    notify(cus_str)
}

pub fn notify<T: Summery>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn return_val() -> impl Summery {
  Str {
    x: String::from("123"),
    y: String::from("123")
  }
}

fn largest<T: PartialOrd>(list: &Vec<T>) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// 生命周期标注  表示两个引用应该在同一生命周期下
// 返回值的生命周期是参数中较短的那个
// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
// 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，所以会报错
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }  
}

// 生命周期 + trait + 泛型的使用
fn longest_with_ann<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str 
where T: Display
{
    // 实现了Display的ann 才能使用这条命令
    println!("ann is {}", ann);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }  
}

pub fn start() {
    main();
}

// fn some_function<T, U>(t: T, u: U) -> i32
// 使用where来声明T的类型
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//   3
// }
