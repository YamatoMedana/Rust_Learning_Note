
//接受命令行参数
use std::env;
use std::process;
use test_derive::{Config,run};

//cargo run the poem.txt
// 12.4. 采用测试驱动开发完善库的功能
// 12.5. 处理环境变量

fn main() {
    let args: Vec<String> = env::args().collect();

    //Config::new 调用并处理错误
    let config = Config::new(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    //使用 if let 来检查 run 是否返回一个 Err 值
    if let Err(_e) = run(config) {
        process::exit(1);
    }

}
