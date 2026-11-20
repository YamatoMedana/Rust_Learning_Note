
//生命周期

/* 
编译错误：返回类型 &str 缺少生命周期标注
原因：有两个独立的输入引用，生命周期省略规则无法确定
      返回的引用应该绑定到 x 的寿命，还是 y 的寿命。
风险：函数体可能返回 x 或 y 中任一引用，若返回的是寿命较短的那个，
      而调用方在它失效后仍使用返回值，就会产生悬垂引用（use-after-free）。
*/
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


/*
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    //ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
    part: &'a str,
}


//生命周期省略
//函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。
//生命周期省略规则：在特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期
//明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，它不会猜测剩余引用的生命周期应该是什么，直接给出错误
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//方法定义中的生命周期标注
//impl 之后和类型名称之后的生命周期参数是必要的，不过因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
impl<'a> ImportantExcerpt1<'a> {
    fn level(&self) -> i32 {
        3
    }
}
//这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期
//接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期
impl<'a> ImportantExcerpt2<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


//静态生命周期
//'static 其生命周期能够存活于整个程序期间
let s: &'static str = "I have a static lifetime.";

//结合泛型类型参数、trait bounds 和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    println!("first_sentence = {}",first_sentence);
    let i = ImportantExcerpt { part: first_sentence };
    println!("part = {}", i.part);



    let word = first_word("hello world");
    println!("word = {}", word);
}