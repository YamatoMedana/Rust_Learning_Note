// 声明 front_of_house 模块。编译器会去 src/ 下找 front_of_house.rs
mod front_of_house;

// 使用 pub use 重导出
// 把原本很长的路径 crate::front_of_house::hosting 提升到根级别
// 这样外部（比如 main.rs）就可以直接通过 crate 名去使用 hosting 了
pub use crate::front_of_house::hosting;

// 本模块内的函数，调用内部模块的方法
pub fn eat_at_restaurant() {
    println!("🍽️ 准备在餐厅就餐...");
    // 这里直接用 hosting 是因为上面已经 use 过了
    hosting::add_to_waitlist();
}