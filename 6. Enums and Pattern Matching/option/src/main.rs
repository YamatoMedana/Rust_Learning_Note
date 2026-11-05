
//Option 是标准库定义的另一个枚举
//Option 类型应用广泛是因为它编码了一个非常普遍的场景，即一个值要么有值要么没值

#![allow(unused)]


//类型靠上下文推断
#[derive(Debug)]
enum MyOption<T> {
    //可以不需要 Option:: 前缀来直接使用 Some 和 None
    Some(T),    //T 可以是任意类型
    None,   //不带载荷
}//这是一个自定义的 Option<T>，它和标准库的 Option<T> 是不同的类型，编译器不允许像一个肯定有效的值那样使用 MyOption<T>



fn main() {
    let some_number = MyOption::Some(5);
    let some_string = MyOption::Some("a string");

    let absent_number: MyOption<i32> = MyOption::None;//显式指定类型
    println!("some_number: {:?}, some_string: {:?}, absent_number: {:?}", some_number, some_string, absent_number);

    //MyOption<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 MyOption<T>
    //T虽然代表任意类型，但是不代表是实际类型，因此不能直接使用 MyOption<T>，必须先解包（unwrap / expect / unwrap_or / match ...）再用
    /*
    let x: i8 = 5;
    let y: MyOption<i8> = MyOption::Some(5);

    let sum = x + y;
     */

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Option<i8> 实现了 Copy，所以 y 可以被重复使用，不会被 move 走
    let sum = x + y.unwrap();       // y 是 None 就 panic
    println!("unwrap 结果: sum = {sum}");
    let sum = x + y.expect("y 必须有值");  // 带信息
    println!("expect 结果: sum = {sum}");

    /*
    反例： 如果使用自定义的 MyOption<T>，就会报错，因为 MyOption<T> 没有实现 Copy trait
    let x: i8 = 5;
    let y: MyOption<i8> = MyOption::Some(5);    // y 是 MyOption<i8>，没有实现 Copy trait，所以 y 会被 move 走
    let sum = x + y.unwrap();       // y 是 None 就 panic
    println!("unwrap 结果: sum = {sum}");
    let sum = x + y.expect("y 必须有值");  // 带信息
    println!("expect 结果: sum = {sum}");
    解决方法：为 MyOption<T> 实现 Copy trait
    impl<T> Copy for MyOption<T> where T: Copy {}
    impl<T> Clone for MyOption<T> where T: Clone {
        fn clone(&self) -> Self {
            match self {
                MyOption::Some(value) => MyOption::Some(value.clone()),
                MyOption::None => MyOption::None,
            }
        }
    }
    也可以使用 match 来解包 MyOption<T>，这样就不会 move
    let x: i8 = 5;
    let y: MyOption<i8> = MyOption::Some(5);
    let sum = match y {
        MyOption::Some(value) => x + value, 
        MyOption::None => x, // 如果 y 是 None，就返回 x
    };
    println!("match 结果: sum = {sum}");
    也可以使用 if let 来解包 MyOption<T>，这样就不会 move
    let x: i8 = 5;
    let y: MyOption<i8> = MyOption::Some(5);
    let sum = if let MyOption::Some(value) = y {
        x + value
    } else {
        x // 如果 y 是 None，就返回 x
    };
    println!("if let 结果: sum = {sum}"); 

    也就是如果使用自定义的 MyOption<T>，就会报错，需要你自己补充Copy、Clone trait的实现，或者使用match、if let来解包MyOption<T>，
    这样就不会move。
    就好比c++，你需要实现++运算符重载，或者使用get()方法来获取值，而不是直接使用对象。

     */

}
