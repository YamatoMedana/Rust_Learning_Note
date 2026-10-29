use std::io::{self, Write};

fn main() {
    let number = input();

    if number <= 5 {
        println!("condition was true");
    } else if number == 6 {
        println!("number is 6");
    } else {
        println!("condition was false");
    }
}

fn input() -> i32 {
    print!("请输入一个整数：");
    io::stdout().flush().expect("无法显示输入提示");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("读取输入失败");

    text.trim().parse().expect("请输入有效的整数")
}
