fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let x = five();
    print!("The value of x is: {}", x);
}

//一个函数有多个参数时，使用逗号分隔
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}


//提供一个返回值的函数，返回值类型通过 -> 指定
//不能返回一个表达式的语句，必须是值
fn five() -> i32 {
    let num  = 5;
    num
}