use std::io::{self, Read};
use std::fs::{self, File};

fn main() {
  match read_file() {
    Ok(s) => println!("content is {}", s),
    Err(e) => println!("err: {}", e)
  }
}

// 这四个函数做的事情都是一样的
fn read_file() -> Result<String, io::Error> {
  let file = File::open("hello.txt");
  let mut file = match file {
    Ok(f) => f,
    Err(e) => return Err(e)
  };
  let mut str = String::new();

  // 传递错误
  match file.read_to_string(&mut str) {
    Ok(_) => Ok(str),
    Err(e) => Err(e)
  }
}

fn read_fileV2() -> Result<String, io::Error> {
  let mut file = File::open("hello.txt")?;
  let mut str = String::new();
  file.read_to_string(&mut str)?;
  Ok(str)
}

fn read_fileV3() -> Result<String, io::Error> {
  let mut str = String::new();
  File::open("hello.txt")?.read_to_string(&mut str)?;
  Ok(str)
}

fn read_fileV4() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}

pub fn start() {
  main()
}