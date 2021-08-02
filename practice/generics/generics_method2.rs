struct Point<T,U> {
    x:T,
    y:U,
}

impl<T,U> Point<T,U> {
    fn mixup<V,W>(self,other:Point<V,W>) -> Point<T,W> {
        Point {
            x:self.x,
            y:other.y
        }
    }
}

fn main() {
    let p1 = Point{x:1,y:2};
    let p2 = Point{x:"不输出我",y:"打印我"};
    println!("调用mixup方法{}",p1.mixup(p2).y);
}