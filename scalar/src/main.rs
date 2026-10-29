
//标值
fn main() 
{

    println!("isize: {} ~ {}", isize::MIN, isize::MAX);
    println!("usize: 0 ~ {}", usize::MAX);
    println!("bits: {}", usize::BITS);

    //整数类型
    {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
        println!("x: {:?}, y: {:?}", x, y);

        let pi = 3.1415926;

        println!("{:.2}", pi);    // 3.14   保留 2 位
        println!("{:.4}", pi);    // 3.1416 保留 4 位
        println!("{:.0}", pi);    // 3      四舍五入到整数
        println!("{:8.2}", pi);   // "    3.14"  宽度 8，右对齐
        println!("{:<8.2}", pi);  // "3.14    "  左对齐
    }

    //数字运算
    {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0

        // remainder
        let remainder = 43 % 5;

        println!("sum: {}, difference: {}, product: {}, quotient: {}, floored: {}, remainder: {}",
            sum, difference, product, quotient, floored, remainder);
    }

    




}
