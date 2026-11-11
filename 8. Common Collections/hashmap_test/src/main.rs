
// 首先 use 标准库中集合部分的 HashMap
use std::collections::HashMap;

fn main() {
    
    // 新建一个哈希 map 并插入一些键值对
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}",scores);

    //用队伍列表和分数列表创建哈希 map
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    //teams.iter()产出Iter<String> → &String
    //initial_scores.iter() 产出Iter<i32> → &i32
    //.zip(...)为拉链，将两个迭代器配对组合起来，Zip → (&String, &i32)
    //.collect()，让他们变成hashmap，HashMap<&String, &i32>
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores:{:?}",scores);

    //哈希 map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    //像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    // 这里 field_name 和 field_value 不再有效，
    map.insert(field_name, field_value);
    println!("map : {:?}",map);

    //访问哈希 map 中的值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //访问哈希 map 中储存的蓝队分数
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}",score);
    //get 返回 Option<V>,有就是some，没有就是none


    //更新哈希 map
    let mut scores = HashMap::new();
    //覆盖一个值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    //只在键没有对应值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    //entry检查是否存在这个键，返回值是一个枚举
    //or_insert方法在键对应的值存在时就返回这个值的可变引用，
    //如果不存在则将参数作为新值插入并返回新值的可变引用
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // 根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    //split_whitespace根据空格来分割
    for word in text.split_whitespace() {
        //entry如果存在那就加1，否则为0.
        // pub enum Entry<'a, K, V> {
        //     Occupied(OccupiedEntry<'a, K, V>),   // key 已存在
        //     Vacant(VacantEntry<'a, K, V>),       // key 不存在
        // }
        //key 已存在,指向那个已有 value 的内部指针
        //key 不存在,记录了 key 和"该往哪插"的位置
        //or_insert与insert相比，不用重复查询，并不会覆盖原来的val
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
