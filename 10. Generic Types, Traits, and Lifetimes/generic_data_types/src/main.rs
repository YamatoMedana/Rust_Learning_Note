

//在函数定义中使用泛型

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

//impl<T, U>声明这个 impl 块引入两个泛型参数T、U
//Point<T, U>使用上面声明的 T、U 来指定“为哪个具体类型实现
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }

    //取第一个点的 x、第二个点的 y，拼成一个新点
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("both_integer={:?}",both_integer);
    println!("both_float={:?}",both_float);
    println!("integer_and_float={:?}",integer_and_float);
    

    let p = Point { x: 5, y: 10.0 };
    println!("p.x = {}", p.x());
    println!("p.y = {:?}", p.y());   

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(5.0);
    println!("integer:{:?}",integer);
    println!("float:{:?}",float);
}

