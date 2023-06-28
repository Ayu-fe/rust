use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    let arr = vec![1, 2, 3, 4, 2];
    // iter()不获取vec的所有权  map()会获取vec的所有权
    let new_arr: Vec<_> = arr.iter().map(|x| x + 1).collect();
    println!("new arr is {:#?}, old arr is {:#?}", new_arr, arr);
    // into_iter()获取vec的所有权  filter不会获取vec的所有权
    let filter_arr: Vec<_> = arr
        .into_iter()
        .filter(|item| *item == 2)
        .collect::<Vec<i32>>();
    println!("filter arr is {:#?}", filter_arr);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, val: u32) -> u32 {
        match self.value.get(&val) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(val);
                self.value.insert(val, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 抽离函数并不是最佳实践 因为没有用到expensive_result的地方也要执行这两条语句
    // let expensive_result =
    //       simulated_expensive_calculation(intensity);

    let expensive_result = |num| {
        // simulated_expensive_calculation(num)
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cache = Cacher::new(expensive_result);

    if intensity < 25 {
        println!("Today, do {} pushups!", cache.value(intensity));
        println!("Next, do {} situps!", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache.value(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn start() {
    main()
}
