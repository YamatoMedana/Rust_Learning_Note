/*
可以使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice,
其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。
slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值
*/


fn main() {
    let mut s = String::from("hello world");

    //引用整个 String 不过带有额外的 [0..5] 部分,对部分 String 的引用。
    let hello = &s[0..5];
    let world = &s[6..11];

    //从索引 0 开始，可以不写两个点号之前的值
    let slice = &s[0..2];
    let slice = &s[..2];

    // slice 包含 String 的最后一个字节，也可以舍弃尾部的数字
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    //可以同时舍弃这两个值来获取整个字符串的 slice
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];


    {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        // s.clear();  //error! 这里的可变引用清空了 String，导致 word 变成了悬垂引用
        println!("the first word is: {}", word);

    }

    //字符串 slice 作为参数
    {
        let my_string = String::from("hello world");

        // `first_word` 接受 `String` 的切片，无论是部分还是全部
        let word = first_word(&my_string[0..6]);
        println!("&my_string[0..6] is: {}", word);

        let word = first_word(&my_string[..]);
        println!("&my_string[..] is: {}", word);

        // `first_word` 也接受 `String` 的引用，
        // 这等同于 `String` 的全部切片
        let word = first_word(&my_string);
        println!("&my_string is: {}", word);

        let my_string_literal = "hello world";

        // `first_word` 接受字符串字面量的切片，无论是部分还是全部
        let hello = first_word(&my_string_literal[0..6]);
        println!("&my_string_literal[0..6] is: {}", hello);
        let word = first_word(&my_string_literal[..]);
        println!("&my_string_literal[..]) is: {}", word);

        // 因为字符串字面值**就是**字符串 slice，
        // 这样写也可以，即不使用 slice 语法！
        let word = first_word(my_string_literal);
        println!("my_string_literalis: {}", word);
    }

    //其他类型的 slice
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        // 这个 slice 包含数组 a 中的第二个和第三个元素,assert_eq! 宏用于断言两个表达式的值相等，如果不相等则会 panic
        //&[2,3] 这个 slice 的类型是 &[i32]，它是一个对 i32 类型数组的引用
        assert_eq!(slice, &[2, 3]);
    }

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    //会返回与底层数据关联的单个值
    &s[..]
}



