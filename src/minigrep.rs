use std::env;
use std::process;
use rust::Config;

fn main() {
    let env: Vec<String> = env::args().collect();
    println!("args is {:#?}", env);

    // let query = &env[1];
    // let file = &env[2];

    let config = Config::new(&env).unwrap_or_else(|err| {
        println!("something is wrong: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust::run(config) {
        println!("read file error: {}", e);
        process::exit(1)
    }

    // let target_file_string = fs::read_to_string(file).expect("something is wrong");
    // println!("poem is \n{}", target_file_string)
}

pub fn start() {
    main()
}
