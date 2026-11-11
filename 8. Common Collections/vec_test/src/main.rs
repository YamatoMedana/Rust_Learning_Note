

fn main() {
    //正常用法
    let mut v1: Vec<i32> = Vec::new();
    //更新 vector
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    //推断出想要存放的类型
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);

    //读取 vector 的元素
    //使用 & 和 [] 返回一个引用
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    //使用 get 方法以索引作为参数来返回一个 Option<&T>
    match v2.get(3) {
        Some(four) => println!("The fourth element is {}", four),
        None => println!("There is no fourth element."),
    }


    //尝试访问一个包含 5 个元素的 vector 的索引 100 处的元素
    //方法一：使用索引访问 vector 的元素
    // let does_not_exist = &v2[100];
    // let does_not_exist = v2.get(100);
    // 方法二，比因为输入错误而使程序崩溃要友好的多
    match v2.get(100) {
        Some(hundred) => println!("The hundredth element is {}", hundred),
        None => println!("There is no hundredth element."),
    }
    // 这两种方式都会 panic，前者会直接 panic，后者会返回 None

    //借用后会修改会发生什么？
    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    // v3.push(6); // 这里会报错，因为在借用 first
    println!("The first element is: {}", first);
    let mut second = &v3[1];
    // second = 10;    //试图把 i32 赋给 &i32
    // //即使你把 10 换成 &10（借用一个临时值），&v3[1] 仍然是不可变借用，不能通过它修改底层元素
    // println!("vec is changed to: {:?}", v3);

    //如果要能修改vec内部的元素，可以使用可变引用
    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &mut v4[0];//这里要加mut，说明 first 是一个可变引用
    *first = 10; //解引用 first，修改它指向的值
    println!("The v4 is changed to: {:?}", v4);

    let mut second = &mut v4[1]; //这里两边加mut，说明 second 是一个可变引用，能够改变指向
    second = &mut v4[2]; //这里将 second 的指向改为 v4[2]，所以 second 现在指向 v4[2]，而不是 v4[1]
    *second = 30; //解引用 second，修改它指向的值
    println!("The v4 is changed to: {:?}", v4);



    //遍历 vector 中的元素
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //获取遍历的序号和数值
    for (num,val) in v.iter().enumerate(){
        println!("索引 {} 的值是 {}", num, val);
    }

    for x in &mut v {
        *x += 1;
        println!("{}", x);
    }
    //这样vec内部元素也被修改了
    println!("{:?}",v);

    //既改元素，又打印序号
    //iter_mut() 和 &mut v 等价，enumerate() 让 i 是 usize（从 0 开始），x 是 &mut i32
    for (i, x) in v.iter_mut().enumerate() {
        *x += 1;
        println!("改变元素,打印序号：索引 {} -> {}", i, x);
    }

    //v[i] 风格,获取序号和数值
    for i in 0..v.len() {
        v[i] += 1;
        println!("v[i] 风格 索引 {} -> {}", i, v[i]);
    }



    //使用枚举来储存多种类型
    {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("{:?}",row);

        //真正读
        for cell in &row{
            match cell {
                SpreadsheetCell::Int(n)      => println!("整数: {n}"),
                SpreadsheetCell::Float(f)    => println!("浮点: {f}"),
                SpreadsheetCell::Text(s)     => println!("文本: {s}"),
            }
        }


    }



}
