

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }
pub fn add_two(a: i32) -> i32 {
    a + 3
}


pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }

    }
}


//控制是否编译。标在模块上时，模块只在运行测试（如 cargo test）时编译
#[cfg(test)]
mod tests {
    //这里 super 指 tests 的父模块
    //* 表示导入其中当前模块可访问的名称
    use super::*;

    //标记一个测试函数。测试运行器会自动发现并执行它
    #[test]
    fn it_works() {
        let result = add(2, 2);
        //参数1为结果，参数2为预期结果
        //不相符就报错
        assert_eq!(result, 4);
    }

    //改变测试的名称并看看这如何改变测试的输出
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /*
    结果

    running 2 tests
    test tests::exploration ... ok
    test tests::it_works ... ok
     */

    //增加一个失败的测试
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    /*
    结果
    running 3 tests
    test tests::exploration ... ok
    test tests::it_works ... ok
    test tests::another ... FAILED
    ....
    单独测试结果和摘要之间多了两个新的部分：第一个部分显示了测试失败的详细原因
     */


    //使用 assert! 宏来检查结果
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
    /*
    结果
    running 4 tests
    test tests::another ... FAILED
    test tests::exploration ... ok
    test tests::it_works ... ok
    test tests::larger_can_hold_smaller ... ok
    ......
    
     */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    /*
    结果
    running 5 tests
    test tests::exploration ... ok
    test tests::larger_can_hold_smaller ... ok
    test tests::it_works ... ok
    test tests::another ... FAILED
    test tests::it_adds_two ... FAILED
    ......
    assertion `left == right` failed
    left: 4
    right: 5
    ......
    
    显示信息 assertion failed: `(left == right)` 并表明 left 是 4 而 right 是 5。
    这个信息有助于我们开始调试：它说 assert_eq! 的 left 参数是 4，
    而 right 参数，也就是 add_two(2) 的结果，是 5
     */

    //自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
    /*
    结果：
    running 6 tests
    test tests::larger_can_hold_smaller ... ok
    test tests::another ... FAILED
    test tests::it_adds_two ... FAILED
    test tests::it_works ... ok
    test tests::greeting_contains_name ... FAILED
    test tests::exploration ... ok
    ......
    thread 'tests::greeting_contains_name' (18992) panicked at src\lib.rs:123:9:
    Greeting did not contain name, value was `Hello!`
    ......

    可以在测试输出中看到所取得的确切的值，这会帮助我们理解真正发生了什么
     */

    //对函数增加另一个属性 should_panic 来实现这些
    //这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。
    //检查 Guess::new 是否按照我们的期望出错的测试
    #[test]
    //告诉测试运行器：这个测试预期会 panic。
    #[should_panic]
    fn greater_than_1() {
        Guess::new(-1);
    }
    /*
    结果
    test tests::greater_than_1 - should panic ... ok
    发生 panic 时，测试结果反而会显示 ok。
    因为加了should_panic属性，所以没有产生 panic的报错。
     */

    //增加expected 参数后
    #[test]
    //expected检查的是 panic 消息是否包含 "指定内容"
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    /*
    结果
    test tests::greater_than_100 - should panic ... FAILED
    ......
    Guess value must be greater than or equal to 100, got 200.
    note: panic did not contain expected string
        panic message: "Guess value must be greater than or equal to 100, got 200."
    expected substring: "Guess value must be less than or equal to 100"
    ......
    失败信息表明测试确实如期望 panic 了
    不过 panic 信息中并没有包含 expected 信息 'Guess value must be less than or equal to 100'。
    而我们得到的 panic 信息是 'Guess value must be greater than or equal to 1, got 200.'。
    这样就可以开始寻找 bug 在哪了！
     */

    //将 Result<T, E> 用于测试
    #[test]
     fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    /*
    返回值类型为 Result<(), String>
    这样编写可以在函数体中使用问号运算符
    注意！【不能在使用 Result<T, E> 的测试中使用 #[should_panic] 注解
    要断言操作返回 Err 值
    不要在 Result<T, E> 值上使用问号运算符
    相反，请使用 assert!(value.is_err())
    】
     */

}
