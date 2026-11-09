

//lib 默认私有，pub 公开
//私有项对定义它的模块及其子模块可见


pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

//在 mod hosting 前添加了 pub 关键字，使其变成公有的
//如果我们可以访问 front_of_house，那我们也可以访问 hosting
//但是 hosting 的 内容（contents） 仍然是私有的；这表明使模块公有并不使其内容也是公有的
//模块上的 pub 关键字只允许其父模块引用它

// mod front_of_house {
//     pub mod hosting {
//         fn add_to_waitlist() {}
//     }
// }



//  crate 根部中定义了 front_of_house 模块。front_of_house 模块不是公有的
// 不过因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中（即，eat_at_restaurant 和 front_of_house 是兄弟）
//  可以从 eat_at_restaurant 中引用 front_of_house
//  使用 pub 标记的 hosting 模块，我们可以访问 hosting 的父模块
//  add_to_waitlist 函数被标记为 pub ，我们可以访问其父模块，自然也可以访问 add_to_waitlist 函数

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释，将会导致编译失败；我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1: {:?}, order2: {:?}", order1, order2);
}


fn serve_order() {}

mod back_of_house {
    //将结构体设为公有
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    //将枚举设为公有
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        //super 表示当前模块的父模块
        super::serve_order();
    }

    //back_of_house::Breakfast 具有私有字段
    impl Breakfast {
        //公共的关联函数
        //通过关联函数，这样才能得到一个 Breakfast 实例，才能修改Breakfast内部的值
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}
}


