//引用与借用



fn main() {
    let s1 = String::from("hello");

    //&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它
    //因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //不可用引用
    // let s = String::from("hello");
    // unchange(&s);  //如变量在默认情况下是不可变的一样，引用也是不可变的。我们无法通过引用修改内容。

    //可变引用
    let mut s = String::from("hello");
    change(&mut s);

    //可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败
    {
        let mut s = String::from("hello");

        //我们不能在同一时间多次将 s 作为可变变量借用
        //第一个可变的借入在 r1 中，并且必须持续到在 println! 中使用它
        //在那个可变引用的创建和它的使用之间，我们又尝试在 r2 中创建另一个可变引用，它借用了与 r1 相同的数据。
        let r1 = &mut s;
        let r2 = &mut s;

        println!("{}, {}", r1, r2);
    }
    //防止同一时间对同一数据进行多个可变引用的限制允许可变性，不过是以一种受限制的方式允许。
    //这个限制的好处是 Rust 可以在编译时就避免数据竞争
}

// & 符号就是 引用，它们允许你使用值但不获取其所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}   // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
    // 所以什么也不会发生


/*
创建一个引用的行为称为 借用
在这里，calculate_length 函数借用了 s 的值，而没有获取其所有权。通过使用引用，我们允许函数使用值，但不获取其所有权。
其中必须mut引用的参数是可变引用，才可以修改引用的值

*/
//不可改引用
// fn unchange(some_string: &String) {
//     some_string.push_str(", world");
// }

//可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


