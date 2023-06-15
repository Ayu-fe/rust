
use std::io;

#[derive(Debug)]
struct Rect {
  width: u32,
  height: u32,
}

// 定义实例下的方法  
impl Rect {
  fn can_hold(&self, other: &Rect) -> bool {
    self.width > other.width
  }
}
// 关联函数 通过::调用 第一个参数不是self
impl Rect {
  fn square(size: u32) -> Rect {
    Rect {
      width: size,
      height: size
    }
  }
}

fn main() {
  // slice
  // let str = String::from("hello world");
  // let first = get_first_word(&str);
  let mut width = String::new();
  let mut height = String::new();

  // io::stdin().read_line(&mut width).expect("error");
  // io::stdin().read_line(&mut height).expect("error");

  let rect = Rect {
    width: 10,
    height: 20
  };
  let rect2 = Rect {
    width: 15,
    height: 20
  };

  println!("rect is {:#?}", rect);
  let res = clu_rect(&rect);
  println!("result is {}", res);
  println!("can hold? {}", rect.can_hold(&rect2));
  let sq = Rect::square(32);

}

fn clu_rect(rect: &Rect) -> u32 {
  rect.height * rect.width
}

fn get_first_word(s: &str) -> &str {
  let byte_arr = s.as_bytes();

  for (i, &item) in byte_arr.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]
    }
  }
  &s[..]
}

pub fn start() {
  main()
}