
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add into front_of_house");
        }
    }
}

mod back_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add into back_of_house");
        }
    }
}

mod house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add into house");
        }
    }
}

//使用 use 将模块引入作用域，绝对路径：从 crate 根一路写下来
//crate (main.rs)
// └── restaurant          ← restaurant.rs 在这一层
//     ├── front_of_house
//     ├── back_of_house
//     └── house
use crate::restaurant::front_of_house::hosting;

//使用 use 和相对路径将模块引入作用域，相对路径：self:: 表示"当前模块 restaurant"
//在这里，同一个作用域不能出现同名的项，因此使用as关键字将模块重命名为 bk_hosting
use self::back_of_house::hosting as bk_hosting;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    bk_hosting::add_to_waitlist();
    house_hosting::add_to_waitlist();
}


//使用 pub use 重导出名称
pub use self::house::hosting as house_hosting;
//当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的
//为了能让自己写的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use

//换个角度看就是，另外一个文件想用内部的模块，使用 pub use 将其导出，其他文件就可以直接使用了
// 例如，使用 pub use 将 house::hosting 模块导出后，其他文件就可以通过 crate::house_hosting::add_to_waitlist() 
//来访问 add_to_waitlist 函数，而不需要知道它原本位于 house::hosting 模块中

//如果不用pub use 导出，其他文件就无法访问 house::hosting 模块中的 add_to_waitlist 函数，因为它是私有的
