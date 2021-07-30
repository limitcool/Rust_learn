#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height >= self.height
    }
    // 这是一个关联函数
    fn square(size:u32) -> Rectangle {
        Rectangle{width:size,height:size}
    }
}

fn main() {
    // 实例化rect1
    let rect1 = Rectangle{width:30,height:50};
    // 实例化rect2
    let rect2 = Rectangle{width:50,height:50};
    println!("RECT1的面积是{}",rect1.area()); // 1500
    println!("RECT2的面积是{}",rect2.area()); // 2500
    println!("RECT1能容纳RECT2{}",rect1.can_hold(&rect2)); // false
    println!("RECT2能容纳RECT1{}",rect2.can_hold(&rect1)); // true
    println!("正方形是{:#?}",Rectangle::square(15)); // 225
    let square1 = Rectangle::square(15);
    println!("正方形的面积是{}",square1.area()); // 225
    println!("调用Recatangle::square正方形的面积是{}",Rectangle::square(15).area());

}
