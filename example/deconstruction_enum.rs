// 需要 'allow' 来消除警告,应为只使用了枚举类型的一种取值。
#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字(而非类型)来指定。
    Red,
    Blue,
    Green,
    // 这些则把'u32' 元组赋予不同的名字,以色彩模型命名。
    RGB(u32,u32,u32),
    HSV(u32,u32,u32),
    HSL(u32,u32,u32),
    CMY(u32,u32,u32),
    CMYK(u32,u32,u32,u32),
}

fn main() {
    let color = Color::RGB(122,17,40);
    // let color = Color::HSV(125,15,40);
    println!("它是什么颜色？");
    // 使用'match'来解构'enum'
    match color {
        Color::Red => println!("The color is Red"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r,g,b) =>
            println!("Red:{},Green:{},Blue:{}!",r,g,b),
        Color::HSV(h,s,v) =>
            println!("Hue:{},Saturation:{},value:{}!",h,s,v),
        Color::HSL(h,s,l) =>
            println!("Hue:{},Saturation:{},Lighness:{}!",h,s,l),
        Color::CMY(c,m,y) =>
            println!("Cyan:{},Magenta:{},Yellow:{}!",c,m,y),
        Color::CMYK(c,m,y,k) =>
            println!("Cyan:{},Magenta:{},Yellow:{},Key(black):{}!",
                c,m,y,k)
        // 不需要其他分支，因为所有情形都已被覆盖
    }
}