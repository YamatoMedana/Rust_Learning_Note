fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

/*
string性能开销就太大了
如果你没明确要求拷贝，我就绝对不悄悄帮你做深拷贝
所以，当你把 String 传给函数时，Rust 默认采用移动语义（Move）
*/
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

/*
在 Rust 里,i32 这种完全存在于栈上、大小固定、极其简单的类型，复制它的成本几乎为零
Rust 为这类基础类型（整数、浮点数、布尔值、字符，以及完全由它们组成的元组等）实现了 Copy trait
*/
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


/*
核心区别来了：为了保证安全（防止双重释放 Double Free），Rust 编译器会直接吊销原变量 s 的使用权。这就叫所有权被 Move（移动）走了
*/