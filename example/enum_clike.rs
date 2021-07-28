// 该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

// 拥有隐式辨别值的enum(从0开始)
enum Number {
    zero,
    one,
    two,
}

// 拥有显式辨别值的enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // 'enum' 可以转换为整型，
    println!("zero is {}",Number::zero as i32);
    println!("one is {}",Number::one as i32);

    println!("rosea are #{:06x}",Color::Red as i32);
    println!("violeta are #{:06x}",Color::Blue as i32);
}