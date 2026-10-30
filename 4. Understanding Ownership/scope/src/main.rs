
//作用域例子
/*
内存在拥有它的变量离开作用域后就被自动释放
当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。
这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop
*/

fn main() {
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 开始有效
        println!("{}", s);

        // let s : String = "hello"; //这样是会报错的
        let s :String = "hello".into(); //这样是可以的
        let s : String = String::from("hello"); //这样是可以的
        println!("{}", s);
    }                       // 此作用域已结束，s 不再有效

    {//双冒号（::）运算符允许我们将特定的 from 函数置于 String 类型的命名空间
        let s = String::from("hello");
        println!("{}", s);
    }

    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() 在字符串后追加字面值
        println!("{}", s); // 将打印 `hello, world!
    }

    //变量与数据交互的方式（一）：移动
    {
        let s1 = String::from("hello");
        let s2 = s1;

        //所有权被转移了，Rust将s1的值移动到s2中，s1不再有效,这种操作称之为move
        // println!("{}, world!", s1);//会报错
        println!("{}, world!", s2);

    }
    
    //变量与数据交互的方式（二）：克隆
    //如果真的需要进行深度复制，也就是深拷贝，则需要用到clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

}
