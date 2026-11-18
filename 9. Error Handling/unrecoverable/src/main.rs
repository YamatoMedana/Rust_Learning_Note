


//panic! 与不可恢复的错误
//[panic!]宏,当执行这个宏时，程序会打印出一个错误信息

fn main() {
    //使用 panic! 的 backtrace
    let v = vec![1, 2, 3];
    // 可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace
    // RUST_BACKTRACE=1 cargo run  （git bash 环境下）
    v[99];

    /*
    thread 'main' (21852) panicked at src\main.rs:12:6:
    index out of bounds: the len is 3 but the index is 99
    stack backtrace:
    0: std::panicking::panic_handler
                at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library\std\src\panicking.rs:679
    1: core::panicking::panic_fmt
                at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library\core\src\panicking.rs:80
    2: core::panicking::panic_bounds_check
                at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library\core\src\panicking.rs:271
    3: core::slice::index::impl$2::index<i32>
                at E:\Rust\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\slice\index.rs:238
    4: alloc::vec::impl$15::index<i32,usize,alloc::alloc::Global>
                at E:\Rust\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\alloc\src\vec\mod.rs:3934
    5: unrecoverable::main
                at .\src\main.rs:12
    6: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                at E:\Rust\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:250
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    这里说了第五条，main.rs 的第12行 有问题，即现在的v[99]
     */
    
}
