

//Result 与可恢复的错误

use std::fs::{File, OpenOptions}; 
use std::io::{self, ErrorKind, Read, Write};

//ErrorKind 是标准库 std::io::ErrorKind 定义的枚举
/*
pub enum ErrorKind {
    NotFound,           // 文件/路径不存在
    PermissionDenied,   // 权限不足
    AlreadyExists,      // 已存在（如 create_new 时文件已存在）
    ConnectionRefused,  // 连接被拒绝
    ConnectionReset,    // 连接被重置
    TimedOut,           // 超时
    UnexpectedEof,      // 意外读到 EOF
    InvalidInput,       // 参数非法
    // ...还有几十个变体
    Other,              // 未分类的兜底
}
*/


fn main() {
    let f = File::open("hello.txt");

    //与 Option 枚举一样
    //Result 枚举和其成员也被导入到了 prelude 中
    //就不需要在 match 分支中的 Ok 和 Err 之前指定 Result::
    let mut f = match f {
        Ok(file) => file,
        //匹配不同的错误,kind()返回一个&ErrorKind 枚举，
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fd) => fd,
                Err(err) => panic!("Problem creating the file: {:?}", err),
            },
            //变量绑定，剩下的任何值，都绑定到这个变量上
            //other_error这个都是自己定义名字，放在最后，表示剩余的类型都绑到这个变量上
            //ErrorKind::Other是标准库的变体
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
        
    };

    // 读取并打印文件内容
    let mut content = String::new();

    //expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数
    //而不像 unwrap 那样使用默认的 panic! 信息
    //因此更多的使用expect
    f.read_to_string(&mut content).expect("读取文件内容失败");
    println!("content = {}", content);


    read_file();

    append_one_liner("增加一行新的\n");

    // 失败时 panic 的简写：unwrap 和 expect
    // unwrap如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    let mut txt = String::new();
    let f1 = File::open("unwrap.txt").unwrap();
    //会显示：called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
    //然后退出程序，无法继续处理错误


}
// 传播（propagating）错误
//调用者可能拥有更多信息或逻辑来决定应该如何处理错误
fn read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    let result = match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };

    //这里不能加分号; 会导致隐式返回的是println的()
    println!("read_username_from_file finish");

    result
}

// 写法 A'：追加写，一行行
fn append_step_by_step(content: &str) -> Result<(), io::Error> {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("hello.txt")?;                      // ? 1：打开
    write!(f, "{}", content)?;                  // ? 2：写入
    Ok(())
}

// 写法 B'：追加写，一行过（整个表达式就是返回值，连 Ok(()) 都省了）
fn append_one_liner(content: &str) -> Result<(), io::Error> {
    write!(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("hello.txt")?,                  // ? 1
        "{}", content
    )                                             // writeln! 本身返回 Result，直接当尾表达式
}