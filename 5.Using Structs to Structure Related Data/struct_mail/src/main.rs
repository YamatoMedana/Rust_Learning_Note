
/*
定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义
在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）

*/


                                                //相比C++的结构体,只是类型声明和字段声明的语法不同,其他的基本概念是一样的
struct User {                                   //struct User {
    active: bool,                                   //bool active;                
    username: String,                               //String username;
    email: String,                                  //String email;
    sign_in_count: u64,                             //unsigned long sign_in_count;
}                                                //}

fn main() {
    let user1 = User {
        //key: value 键-值对的形式提供字段
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };//不可变，内部不可修改

    let mut user2 = User {
        //将user1的字段值复制给user2,但是user2是可变的，user1内容已作废，
        //不可给user3使用user1的字段值，因为user1已经被移动了
        ..user1
    }; //创建一个可变的结构体实例
    user2.email = String::from("another@example.com");
    user2.username = String::from("anotherusername456");
    user2.active = false;
    user2.sign_in_count = 0;

    //通过结构体更新语法
    let mut user3 = User {
        active: user2.active,
        //user2.email 还能使用，user2.username 不能了。user2 作为整体已经"部分失效"
        //导致printlin!中user2.email无法使用
        // username: user2.username,//会报错

        //字符串类型因为重新赋值了，没有使用到user2.email, 所以user2还能使用
        username: String::from("newusername789"), //重新赋值
        email: String::from("another@example.com"),
        sign_in_count: user2.sign_in_count,
    };

    //总结：除了字符串类的类型，其他类型的字段可以直接使用user2的字段值，字符串类的字段如果重新赋值了，就不能再使用user2的字段值了
    //因为字符串类的类型是堆分配的，重新赋值会移动所有权，导致原来的变量失效
    println!("user2: email={}, username={}, active={}, sign_in_count={}", user2.email, user2.username, user2.active, user2.sign_in_count);
    println!("user3: email={}, username={}, active={}, sign_in_count={}", user3.email, user3.username, user3.active, user3.sign_in_count);

    tuple_struct();
}

fn build_user(email: String, username: String) -> User {
    User {
        //严谨的写法
        // email: email,
        // username: username,

        //字段初始化简写语法
        email, //如果函数参数的名字和结构体字段的名字相同，可以使用这种简写方式
        username,
        active: true,
        sign_in_count: 1,
    }
}

//使用没有命名字段的元组结构体来创建不同的类型

#[derive(Debug)] struct Color(i32, i32, i32);

#[derive(Debug)] struct Point(i32, i32, i32);

fn tuple_struct(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black:{:?}",black);
    println!("origin:{:?}",origin);
}


//没有任何字段的类单元结构体
fn unit-like structs(){
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}