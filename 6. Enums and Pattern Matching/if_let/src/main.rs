
//if let 语法让我们以一种不那么冗长的方式结合 if 和 let

#[derive(Debug, Clone, Copy)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn main() {
    // ① 必须先创建 coin 实例，否则后面 match / if let 都找不到 coin
    let coin = Coin::Quarter(UsState::Alaska);
    let some_u8_value = Some(0u8);


    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    //  if let 获取通过等号分隔的一个模式和一个表达式
    //  它的工作方式与 match 相同
    if let Some(3) = some_u8_value {
        println!("three");
    }

//可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值
//直接判断有没有匹配到 Some(3)，如果匹配到了就执行代码块，如果没有匹配到就忽略


    //可以在 if let 中包含一个 else
    //else 块中的代码与 match 表达式中的 _ 分支块中的代码相同
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }



}
