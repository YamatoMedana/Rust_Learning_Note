//数组学习


fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months: {:?}", months);

    //使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    //如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号，然后在方括号中指定数组的长度
    let a = [0; 5];
    println!("a: {:?}", a);

    //使用索引访问数组的元素
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);
}
