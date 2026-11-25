
//接受命令行参数
use std::env;
use std::process;
use std::fs;
use comp_cfg_val::{Config,run};

//cargo run the poem.txt
// 12.3. 重构以改进模块化与错误处理

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    //Config::new 调用并处理错误
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("config = {:?}",config);

    //从 main 提取逻辑
    // run(config);

    //使用 if let 来检查 run 是否返回一个 Err 值
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

    
}



pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    //parse_config 的签名表明它现在返回一个 Config 值
    Config { query, filename }
}