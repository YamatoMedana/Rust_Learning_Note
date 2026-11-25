
use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

//创建一个 Config 的构造函数
impl Config {
    // fn new(args: &[String]) -> Config {
    //从 new 中返回 Result 而不是调用 panic!
    //成功时带有一个 Config 实例而在出现错误时带有一个 &'static str
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        //检查参数数量
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // Config { query, filename }
        Ok(Config { query, filename })
    }
}

//run 函数包含了 main 中从读取文件开始的剩余的所有逻辑
// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }


//从 run 函数中返回错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}