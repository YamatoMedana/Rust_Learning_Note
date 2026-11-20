
use std::fmt::{self, Display};


pub trait Summary {
    // fn summarize_author(&self) -> String;//如果写了就要必须实现，堪比纯虚函数
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//这种写法，有点像是将Summary 嵌入到 结构体NewsArticle中
//作为一个类似于c++虚函数的存在
//impl 理解成“给类型贴标签”
/*
struct 定义的是“这个类型有什么数据”（名词的构成）

impl Trait for Type 定义的是“这个类型能做什么”（动词的能力）

trait 本身是一份“能力清单/合同”，impl 是“签字承诺履行合同”
*/
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.author)
    // }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.author)
    // }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 复用 Summary 的实现，保证格式永远一致
        write!(f, "{}", self.summarize())
    }
}