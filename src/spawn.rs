use std::thread;
use std::time::Duration;

fn main() {
  let v = 12;

  let new_spawn = thread::spawn(move || {
    // 新线程的闭包无法直接使用环境外的数据 因为rust不知道新线程会持续多久 外面的数据可能会失效 所以应该把他的所有权移进来
    for i in 1..10 {
      println!("thi is spawn, result is {}, v is {}", i, v);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // 主线程结束 新线程也会结束 可以使用join来等待所有线程结束 会阻塞当前线程直到 new_spawn 所代表的线程结束
  // 因为会阻塞当前线程  所以join的位置也很重要
  for i in 1..5 {
    println!("thi is main, result is {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  new_spawn.join().unwrap()
}

pub fn start() {
  main();
}