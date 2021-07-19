fn main(){
    let mut x = 5;
    println!("这个值是：{}",x);
    x = 6;
    println!("这个值是：{}",x);
    // 定义常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量是：{}",MAX_POINTS);
    let y = 5;
    let y = y+1;
    let y = y*2;
    println!("Y的是值是{}",y);
    // 浮点数默认类型为f64,f32是单精度,f64是双精度
    let z = 2.0; // f64
    let a: f32 = 3.0; // f32
    println!("z的值是{}，a的值是{}",z,a) 
    // 加法
    let sum = 5 + 10;
    println!("5+10的和为{}",sum);
    // 减法
    let difference = 95.5 - 4.3;
    println!("95.5-4.3的差为{}",difference);
    // 乘法
    let product = 4 * 30;
    println!("4X30的积为{}",product);
    // 除法
    let remainder = 43 % 5;
}