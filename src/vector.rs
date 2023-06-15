use std::collections::{self, HashMap};
struct Sp {
    first: i32,
    second: String,
}

fn main() {
    // vec_demo();
    // string_demo();
    // hash_map_demo();
    let mut arr = vec![1, 3, 4, 2, 3, 2, 2, 3];
    println!("max is {:#?}", clu_list(&mut arr, CluType::More));
}

fn vec_demo() {
    let arr: Vec<i32> = Vec::new();

    // 使用vec宏
    let mut arr2 = vec![1, 2, 3];
    arr2.push(4);
    println!("arr is {:#?}", arr2);

    // 通过索引读取vec 如果不存在会引发panic
    let third = &arr2[2];
    // 通过get并不会导致这种情况 会返回none
    match arr2.get(2) {
        Some(third) => println!("this is third!"),
        None => println!("nothing"),
    }

    // 遍历vec
    for i in &mut arr2 {
        // 取值
        *i += 50
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 通过枚举类型 在vec中创建多种类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let row2 = vec![Sp {
        first: 12,
        second: String::from("123"),
    }];
}

fn string_demo() {
    let data = "hellow";
    let tmp = data.to_string();
    let str1 = String::from("this");
    let str2 = String::from("is");

    // 字符串相加 fn add(string, &str) {}
    // &String 会被强转成 &str &str2[..]
    // str1所有权被移动了
    // let str3 = str1 + &str2;

    // 可以使用format宏来解决这个问题
    let str4 = format!("{}-{}", str1, str2);

    // 遍历字符串
    // 获取原始字节 str4.bytes()
    for i in str4.chars() {
        println!("{}", i)
    }
}

fn hash_map_demo() {
    let mut score = HashMap::new();
    score.insert(String::from("A"), 30);
    score.insert(String::from("B"), 50);

    let team = vec![String::from("blue"), String::from("red")];
    let team_score = vec![10, 15];
    // 使用 zip 方法来创建一个元组的 vector;
    // 使用 collect 方法将这个元组 vector 转换成一个 HashMap
    let yuanzu_vec = team.iter().zip(team_score.iter());
    println!("{:#?}", yuanzu_vec);
    let team_map: HashMap<_, _> = yuanzu_vec.collect();

    // 取值
    let team_blue = team_map.get(&String::from("blue"));
    //   match team_blue {
    //       Some(a) => a,
    //       None =>
    //   }
    // 遍历
    for (key, value) in team_map {
        println!("{}-{}", key, value);
    }

    // 检查是否有某个值
    score.entry(String::from("A")).or_insert(40);
    let tmp_arr = vec![11, 11, 3, 4, 3];
    let fin_arr = arr_check(&tmp_arr);
    println!("arr: {:#?}", fin_arr);
    println!("map: {:#?}", check_str(&String::from("hello world hello")));
}

fn arr_check(arr: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in arr {
        map.entry(i).or_insert(true);
    }
    let mut new_arr: Vec<i32> = Vec::new();
    for (key, _) in map {
        new_arr.push(*key);
    }
    new_arr
}

fn check_str(str: &String) -> HashMap<&str, i32> {
    // 统计字符串中每个单词出现的次数
    let mut map: HashMap<&str, i32> = HashMap::new();
    for key in str.split_whitespace() {
        // or_insert 返回键的值的可变引用
        let count = map.entry(key).or_insert(0);
        *count += 1;
    }
    map
}

enum CluType {
    Average,
    Middle,
    More,
}

fn clu_list(arr: &mut Vec<i32>, clu_type: CluType) -> Vec<i32> {
    //  中位数和众数
    match clu_type {
        CluType::Average => {
            // 计算平均数
            let mut sum = 0;
            let temp_arr = arr;
            let length = temp_arr.len();
            for item in temp_arr {
                sum += *item
            }
            vec![sum / length as i32]
        }
        CluType::Middle => {
            // 计算中位数
            let temp_arr = arr;
            println!("sort arr is {:#?}", temp_arr);
            let length = temp_arr.len();
            let mid = length / 2;
            if length % 2 == 1 {
                // 奇数 
                return vec![temp_arr[mid]]
            } else {
                // 取中间俩求平均
                let left = temp_arr[mid - 1];
                let right = temp_arr[mid + 1];
                return vec![(left + right) / 2];
            }
        }
        CluType::More => {
            // 计算众数
            let mut map = HashMap::new();
            let mut result = Vec::new();
            for item in arr {
                let count = map.entry(item).or_insert(0);
                *count += 1;
            }
            let mut max_count = 0;
            for (key, value) in map {
                println!("key-{} value-{} max-count-{}", key, value, max_count);
                if max_count < value {
                    result = Vec::new();
                    result.push(*key);
                    max_count = value;
                } else if max_count == value {
                    result.push(*key);
                }
            }
            result
        }
    }
}

pub fn start() {
    main()
}
