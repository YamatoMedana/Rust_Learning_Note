


fn main() {

    //新建字符串
    {    
        let mut s = String::new();
        //希望一开始就有这个字符串,可以使用 to_string 方法
        let data = "initial contents";
        let s = data.to_string();

        // 该方法也可直接用于字符串字面量：
        let s = "initial contents".to_string();
    
        println!("string = {}", s);

        //使用 to_string 方法从字符串字面量创建 String
        //字符串是 UTF-8 编码的
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }


    //更新字符串
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("s = {}",s);

        //使用 push_str 方法向 String 附加字符串 slice
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);


        //将字符串 slice 的内容附加到 String 后使用它
        let mut s = String::from("lo");
        s.push('l');
        println!("s is {}", s);

        //使用 + 运算符或 format! 宏拼接字符串
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
        //因为标准库中
        // impl Add<&str> for String {
        //     type Output = String;

        //     fn add(self, other: &str) -> String {
        //         // self 是 String（按值），other 是 &str（借用）
        //         // 把 other 的内容追加到 self 的缓冲区后面，返回 self
        //         ...
        //     }
        // }
        //所以 s1为主体，s1 + &s2展开成s1.add(&s2)
        println!("s1+s2=s3 is {}", s3);

        //如果想要级联多个字符串，+ 的行为就显得笨重了
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s1+s2+s3 : {}",s);

        //因此可以使用 format! 宏
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s1+s2+s3 : {}",s);


    }

    //索引字符串
    {
        let s1 = String::from("hello");
        // let h = s1[0];  //会报错，字符串不支持索引
        //为什么不支持呢？因为String 是一个 Vec<u8> 的封装

        //真的要访问字符串
        //那么有三种方式

        //按字节访问 → as_bytes()
        let s1 = String::from("hello");
        for (i, byte) in s1.as_bytes().iter().enumerate() {
            println!("as_bytes()字节索引:{}, 字节值:{} ({})", i, byte, *byte as char);
        }

        //按字符访问 → chars().enumerate()
        let s1 = String::from("hello");
        for (i, ch) in s1.chars().enumerate() {
            println!("chars().enumerate()字符序号:{}, 字符:{}", i, ch);
        }

        //按字符 + 字节位置 → char_indices()
        let s1 = String::from("hello");
        for (byte_pos, ch) in s1.char_indices() {
            println!("char_indices字节位置:{}, 字符:{}", byte_pos, ch);
        }

        //注意：s1.len() 是字节数，不是字符数

    }
}
