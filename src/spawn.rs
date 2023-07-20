use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
  // new_spawn();

  // new_channel();

  new_mutex();
}

fn new_mutex() {
  // 使用Mutex来创建一个可以在线程间共享的状态
  // 线程想要修改数据 需要通过lock()来获取锁
  // 多线程使用共享状态需要使用 Arc 来保证多所有者 不使用Rc是因为Rc不能在多线程中保证安全性
  // Arc：原子引用计数
  let counter = Arc::new(Mutex::new(1));
  let mut pown = vec![];

  for _ in 1..10 {
    let clone_counter = Arc::clone(&counter);
    let thread = thread::spawn(move || {
      let mut cur_val = clone_counter.lock().unwrap();
      *cur_val += 1;
    });
    pown.push(thread);
  }

  for item in pown {
    item.join().unwrap();
  }

  println!("result is {}", counter.lock().unwrap());
}

fn new_channel() {
  // 一条通道是 多生产者  单消费者  
  // 创建一条通道
  let (tx, rx) = mpsc::channel();

  // 通过clone创建多个生产者
  let tx1 = tx.clone();

  // 开启一条线程并发送
  thread::spawn(move || {
    let val = String::from("hi");
    let val_arr = vec![String::from("hi"), String::from("ni"), String::from("hao")];

    for item in val_arr {
      tx.send(item).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
    // val的所有权会被转移
    // tx.send(val).unwrap();
  });

  // 在主线程接收
  // let res = rx.recv().unwrap();

  // rx自己也可以使一个迭代器
  for item in rx {
    println!("res is {}", item);
  }
  
}

fn new_spawn() {
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