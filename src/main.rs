// 主入口
mod guess_game;
mod reference;
mod struct_slice;
mod enum_if;
mod vector;
mod panic_result;
mod trait_life;
mod minigrep;
mod closure;
mod smart_point;
mod spawn;

fn main() {
  // 猜数字游戏
  // guess_game::start();

  // 变量相关
  // reference::start();

  // 结构体和切片
  // struct_slice::start();

  // 枚举和if let控制流 和crate
  // enum_if::start();

  // vector和常见集合
  // vector::start();

  // 错误处理
  // panic_result::start();

  // trait 泛型和生命周期
  // trait_life::start();

  // minigrep
  // minigrep::start();

  // 闭包&迭代器
  // closure::start();

  // 智能指针
  // smart_point::start();

  // 线程 无畏并发
  spawn::start();
}