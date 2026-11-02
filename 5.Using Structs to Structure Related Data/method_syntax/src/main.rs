#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

//impl 是 implementation 的缩写），这个 impl 块中的所有内容都将与 Rectangle 类型相关联,impl 像 C++ 的类外成员函数定义
//将参数和函数体中其他地方的对应参数改成 self，有点像c++里面的this指针，self是一个特殊的类型，它代表了调用这个方法的结构体实例
impl Rectangle {
    //&self 实际上是 self: &Self 的缩写
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
//一个类型可以有多个 impl 块
impl Rectangle {
    //self 是方法的接收者，方法完全可以有别的参数
    //这种写法类似于C++里面的成员函数，self是一个特殊的类型，它代表了调用这个方法的结构体实例
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    //带 self 的 fn → C++ 非静态成员函数
    fn distance_to(&self, other: &Rectangle) -> i32 {
        let dx = (self.x - other.x) as i32;
        let dy = (self.y - other.y) as i32;
        (dx * dx + dy * dy) as i32
    }

    //不带 self 的 fn → C++ 静态成员函数
    fn origin() -> Rectangle { 
        Rectangle { x: 0, y: 0, width: 0, height: 0 }
    }

    fn print_info(&self) {
        println!("Rectangle: x={}, y={}, width={}, height={}", self.x, self.y, self.width, self.height);
    }
}

fn main() {
    let mut rect1 = Rectangle {
        x: 0,
        y: 0,
        width: 30,
        height: 50,
    };

    //输出整个结构体的名字和内部的字段值，Debug trait 允许使用 {:?} 来打印结构体
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //impl 内 fn 的真正特殊之处
    //impl 内的带 self 的 fn 参与 Rust 的自动引用/解引用
    //// 编译器自动把 rect1 变成 &rect1 传给 translate
    rect1.translate(10, 20); //rect1.translate(&mut rect1, 10, 20); 自动引用
    rect1.print_info(); //rect1.print_info(&rect1); 自动引用
}
