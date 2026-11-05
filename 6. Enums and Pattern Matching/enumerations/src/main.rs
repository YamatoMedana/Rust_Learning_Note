
//定义枚举

enum IpAddrKind {
    //IpAddr 枚举的新定义表明了关联了 String 值
    //每个成员可以处理不同类型和数量的数据
    V4(u8, u8, u8, u8),
    V6(String), 
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//内嵌了多种多样的类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//枚举和结构体还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

#![allow(unused)]
fn main() {

/*  冗余写法，不过为了表示枚举成员可以关联不同类型和数量的数据，下面的写法是为了说明这个特性
    let home = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),          // 4 个 u8
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),   // 1 个 String
        address: String::from("::1"),
    };
*/
    //地道写法
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();  
}

