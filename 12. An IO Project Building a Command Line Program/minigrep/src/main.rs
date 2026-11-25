
//接受命令行参数
use std::env;
use std::fs;


// 12.1. 接受命令行参数
// 12.2. 读取文件

fn main() {
    //读取参数值
    //std::env::args返回一个传递给程序的命令行参数的 迭代器
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    //提取参数解析器
    let (query, filename) = parse_config(&args);
    println!("query={}, filename={}",query,filename);


}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}