mod restaurant;

use std::collections::HashMap;

// use std::cmp::Ordering;
// use std::io;
//嵌套路径来消除大量的 use 行，使用嵌套路径将相同的项在一行中引入作用域
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
//两个路径的相同部分是 std::io，可以在嵌套路径中使用 self
// use std::io::{self, Write};

//通过 glob 运算符将所有的公有定义引入作用域
use std::collections::*;

//由于已经在restaurant.rs中使用 pub use 将 house::hosting 模块导出为 house_hosting，
//因此在 main.rs 中可以直接使用 house_hosting::add_to_waitlist() 来访问 add_to_waitlist 函数，
//而不需要知道它原本位于 house::hosting 模块中
use crate::restaurant::house_hosting::add_to_waitlist;

//如果没有用到pub use 导出，其他文件就无法访问 house::hosting 模块中的 add_to_waitlist 函数，因为它是私有的
//use crate::restaurant::front_of_house::hosting; //会报错，说front_of_house是一个private module

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    add_to_waitlist();
    restaurant::eat_at_restaurant();
}
