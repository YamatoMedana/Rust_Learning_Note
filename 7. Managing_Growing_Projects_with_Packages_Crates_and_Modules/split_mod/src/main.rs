// 使用你的项目名 split_mod 来引入库里的东西
use split_mod::eat_at_restaurant;

//由于 lib.rs 中使用了 pub use 将 front_of_house::hosting 模块导出为 hosting，也就是让main知道有一个split_mod内有一个 hosting 模块，
//因此在 main.rs 中可以直接使用 hosting::add_to_waitlist() 来访问 add_to_waitlist 函数，
//而不需要知道它原本位于 front_of_house::hosting 模块中
use split_mod::hosting;

fn main() {
    println!("--- 从 main 函数开始运行 ---");
    
    eat_at_restaurant();
    
    // 这里可以直接用 hosting，是因为 lib.rs 里写了 pub use
    hosting::add_to_waitlist();
}