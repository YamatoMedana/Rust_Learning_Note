//变量和常量


//Rust 常量的命名全部字母都使用大写,下划线分隔单词
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    //y=5+1=6
    let y = y + 1; 
    {
        //y=6*2=12
        let y = y * 2;  
        println!("The value of y in the inner scope is: {}", y);
    }
    //The value of y in the outer scope is: 6 类似于用一层层来滤镜遮蔽，跳出{}恢复原来的数值
    println!("The value of y in the outer scope is: {}", y);
}