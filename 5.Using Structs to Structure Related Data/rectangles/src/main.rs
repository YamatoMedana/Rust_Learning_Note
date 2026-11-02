
// 外部属性 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    /* 
    println! 宏能处理很多类型的格式，不过，{} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。
    对于结构体，println! 应该用来输出的格式是不明确的，
    因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？
    */
    // println!("rect1 is {:?}", rect1);//会报错

    //这样就会输出整个结构体的名字和内部的字段值，Debug trait 允许使用 {:?} 来打印结构体
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
