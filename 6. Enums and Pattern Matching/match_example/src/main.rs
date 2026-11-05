use rand::RngExt;

#[derive(Debug, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

//绑定值的模式
//匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值
#[derive(Debug, Clone, Copy)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //如果模式匹配了这个值，这个模式相关联的代码将被执行
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // => 运算符将模式和将要运行的代码分开
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//通配模式和 _ 占位符
fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}
fn reroll(){
    let mut rng = rand::rng();          // rand 0.9+
    let n: u8 = rng.random_range(1..=6);
    println!("Rerolled: {n}");
}

fn dice_roll(num: u8) {
    let dice_roll = num;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}



fn main() {
    // ② 构造带参数变体：必须传 UsState
    let c = Coin::Quarter(UsState::Alaska);

    // ② 调用函数，拿到返回值
    let cents = value_in_cents(c);
    println!("{:?} 值 {cents} 美分", c);   // 需要 #[derive(Debug)]

    // ③ 也可以直接传，或者对多个值依次调用
    println!("{}", value_in_cents(Coin::Penny));   // 会打印 Lucky penny! 再打印 1
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}, six = {:?}, none = {:?}", five, six, none);

    dice_roll(3);
    dice_roll(7);
    dice_roll(2);

}
