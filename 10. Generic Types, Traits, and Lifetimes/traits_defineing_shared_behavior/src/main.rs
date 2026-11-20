use traits_defineing_shared_behavior::{NewsArticle, Summary, Tweet};
use std::fmt::{Debug, Display};



pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}//等价于下面这个函数


//Trait Bound 语法
// T: Summary 是一个泛型约束（trait bound）。它的意思是：“调用者传进来的 T，必须在这张实现登记表里，有 Summary for T 这一条
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

//实现获取两个Summary的参数
pub fn notify2(item1: &impl Summary, item2: &impl Summary){
    println!("notify2 news! {}->{}", item1.summarize(),item2.summarize());
}

//如果要强制它们都是相同类型
//只有在使用 trait bound 才行
//pub fn notify<T: Summary>(item1: T, item2: T)

//通过 + 指定多个 trait bound
pub fn notify3(item: &(impl Summary + Display)) {
    println!("notify3: {}", item.summarize());
    println!("notify3_raw: {}", item);
}
// 语法也适用于泛型的 trait bound：
// pub fn notify<T: Summary + Display>(item: T)


// 通过 where 简化 trait bound
fn where_notify<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    let _t2 = t.clone();   // 用到 Clone
    let _u2 = u.clone();   // 用到 Clone
    println!("where_notify_display:{}", t);      // 用到 Display
    println!("where_notify_debug{:?}", u);    // 用到 Debug
    42
}

//还可以像这样使用 where 从句
fn where_bound_notify<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    let _t2 = t.clone();   // 用到 Clone
    let _u2 = u.clone();   // 用到 Clone
    println!("where_notify_display:{}", t);      // 用到 Display
    println!("where_notify_debug{:?}", u);    // 用到 Debug
    0
}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}



fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("New article available! {}", article.summarize());
    println!("New tweet available! {}", tweet.summarize());

    notify(&tweet);

    notify2(&tweet,&article);

    notify3(&tweet);

    where_notify(42, "hello");

    where_bound_notify(42, String::from("world"));

    println!("returns_summarizable = {:?}",returns_summarizable().summarize());
}