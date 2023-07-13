use crate::smart_point::List::{Cons, Nil};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

struct Data {
    value: i32,
}

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // 隐式解引用强制转换 B是A的关联类型 且A实现了deref的trait 就可以强制转换
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    hello(&String::from("a"));

    // Rc可以让一个数据拥有多个所有权
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 克隆 Rc<T> 会增加引用计数
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    // RefCell提供了内部可变引用  Rc<RefCell<T>> 可以实现多个所有权可变引用
    // RefCell提供了内部可变引用  比如这个case  num就是一个外部不可变的数据
    // let num = RefCell::new(42);
    // add(&num);
    // println!("num is {:?}", num);

    // 而这个case  num就是一个外部可变的数据
    // let mut num_mut = 42;
    // add_mut(&mut num_mut);
    // println!("num is {:?}", num_mut);

    rc();
}

fn rc() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    // *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn add(v: &RefCell<i32>) {
    let mut b = v.borrow_mut();
    *b += 1;
    println!("v is {}", b)
}

fn add_mut(v: &mut i32) {
    *v += 1;
    println!("v us {}", v)
}

fn hello(s: &str) {
    println!("{}", s)
}

pub fn start() {
    main()
}
