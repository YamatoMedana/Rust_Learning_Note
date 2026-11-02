use std::io::{self, Write};
use std::collections::HashMap;

//控制流

fn main() {
    ifeample();
    loopexample();
    whileexample();
    forexample();
}

fn ifeample() {
    let number = input();

    if number <= 5 {
        println!("condition was true");
    } else if number == 6 {
        println!("number is 6");
    } else {
        println!("condition was false");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

/*
在一个循环上指定一个循环标签（loop label），
然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
*/

fn loopexample() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}\n\n\n", count);

    //从循环返回：用于停止循环的 break 表达式添加你想要返回的值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

/*
while 条件循环
当条件为真，执行循环。当条件不再为真，调用 break 停止循环
*/
fn whileexample() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

/*
使用 for 遍历集合
在 Rust 里，for x in collection 实际上是 for x in collection.into_iter() 的简写。

for x in &vec 相当于 vec.iter()（产生 &T）

for x in &mut vec 相当于 vec.iter_mut()（产生 &mut T）

for x in vec 相当于 vec.into_iter()（产生 T）
*/
fn forexample() {
    //使用 for 循环遍历集合中的元素
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    //使用 for 循环来倒计时的例子，rev，用来反转区间（range）
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //下面是多种用法
    //1. 基础数字范围 (.. 和 ..=)
    {
        print!("基础数字范围 (.. 和 ..=)：\n");
        // 1. 左闭右开：循环 0, 1, 2, 3, 4 （不包含 5）
        for i in 0..5 {
            println!("{}", i);
        }

        // 2. 全闭：循环 1, 2, 3, 4, 5 （包含 5）
        for i in 1..=5 {
            println!("{}", i);
        }

        // 3. 反向循环（使用 .rev()）
        for i in (1..=3).rev() {
            println!("倒计时: {}", i); // 输出 3, 2, 1
        }

        // 4. 带步长循环（使用 .step_by()，类似 i += 2）
        for i in (0..10).step_by(2) {
            println!("{}", i); // 输出 0, 2, 4, 6, 8
        }
    }
    //2. 遍历集合的三种所有权模式
    {
        print!("遍历集合的三种所有权模式：\n");
        let arr = vec![10, 20, 30];

        // 写法 A：只读引用（不获取所有权，最常用）
        // 相当于遍历 &arr
        for item in &arr {
            println!("{}", item); // item 的类型是 &i32
        }

        // 写法 B：可变引用（修改集合内的元素）
        let mut arr2 = vec![1, 2, 3];
        for item in &mut arr2 {
            *item += 10; // 解引用后修改
        }
        println!("{:?}", arr2); // [11, 12, 13]

        // 写法 C：获取所有权（消费集合，循环后原集合失效）
        for item in arr {
            println!("{}", item); // item 的类型是 i32
        }
        // println!("{:?}", arr); // ❌ 报错：arr 被移动（move）了，不能再用
    }

    //3. 需要索引时 (.enumerate())
    {
        print!("需要索引时 (.enumerate())：\n");
        let names = vec!["Alice", "Bob", "Charlie"];

        // 自动加上索引
        for (index, name) in names.iter().enumerate() {
            println!("第 {} 个人是 {}", index, name);
        }
        // 输出：
        // 第 0 个人是 Alice
        // 第 1 个人是 Bob
        // 第 2 个人是 Charlie
    }

    //4. 遍历字典（HashMap / BTreeMap）
    {
        print!("遍历字典（HashMap / BTreeMap）：\n");
        let mut scores = HashMap::new();
        scores.insert("Blue", 10);
        scores.insert("Red", 50);

        // 拿到键和值（只读）
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

    }

    //5. 字符串遍历
    {
        print!("字符串遍历：\n");
        let text = "你好, Rust";

        // 按 Unicode 字符遍历
        for c in text.chars() {
            print!("{}\t ", c); // 你 好 ,   R u s t
        }

        print!("\n");
        // 按底层字节遍历
        for b in text.bytes() {
            println!("{}", b); // 输出各种数字 (u8)
        }
    }

    //6. 链式调用（迭代器适配器）
    {
        print!("链式调用（迭代器适配器）：\n");
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // 过滤出偶数，然后乘以 10，最后再循环
        for n in numbers.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 10) {
            println!("{}", n); // 输出 20, 40, 60
        }

    }


}


fn input() -> i32 {
    print!("请输入一个整数：");
    io::stdout().flush().expect("无法显示输入提示");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("读取输入失败");

    text.trim().parse().expect("请输入有效的整数")
}
