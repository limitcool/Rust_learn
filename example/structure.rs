#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32,f32);

// 带有两个字段的(field)的结构体
struct Point{
    x:f32,
    y:f32,
}

// 结构体可以做为另一个结构体的字段
#[allow(dead_code)]
struct Recangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // 使用简单的写法初始化字段，并创建结构体。
    let name = "Peter";
    let age = 27;
    let peter = Person{name,age};

    // 以Debug方式打印结构体
    println!("{:?}",peter);
    // 实例化结构体 'Point'
    let point: Point = Point{x:0.3,y:0.4};
    // 访问 point 的字段
    println!("点坐标:({},{})",point.x,point.y);

    // 使用结构体更新语法创建新的point,这样可以使用之前的 point的字段
    let new_point = Point {x:0.1, ..point };

    // 'new_point.y' 与 'point.y'的值一样,因为这个字段就是从'point'中来的
    println!("第二个点坐标({},{})",new_point.x,new_point.y);

    // 使用'let' 绑定来解构point
    let Point {x: my_x,y:my_y} = point;
    let _rectangle = Recangle {
        // 结构体的实例化也是一个表达式
        p1: Point {x: my_y,y:my_x},
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;
    // 实例化一个元组结构体
    let pair = Pair(1,0.1);

    // 访问元组结构体的字段
    println!("pair中包含{:?}和{:?}",pair.0,pair.1);
    // 解构一个元组结构体
    let Pair(integer, _) = pair;
    println!("pair中包含{:?}",integer);
}