// 结构体中的泛型
struct Point<T> {
    x:T,
    y:T,
}
fn main() {
    let integer = Point {x:5,y:10};
    let float = Point {x:10.,y:4.0};
}
