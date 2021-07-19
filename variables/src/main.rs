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
    println!("z的值是{}，a的值是{}",z,a); 
    // 加法
    let sum = 5 + 10;
    println!("5+10的和为{}",sum);
    // 减法
    let difference = 95.5 - 4.3;
    println!("95.5减去4.3的差为{}",difference);
    // 乘法
    let product = 4 * 30;
    println!("4X30的积为{}",product);
    // 除法
    let quotient = 56.7 / 32.2;
    println!("56.7除以32.2的商为{}",quotient);
    // 取余
    let remainder = 43 % 5;
    println!("43对5取余的余数为{}",remainder);
    // 布尔值
    let t = true;
    let f:bool = false; // 显式指定类型注解
    // 字符类型
    let c = 'z';
    println!("c的值是{}",c);
    let d = 'ℤ';
    println!("d的值是{}",d);
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat的值是{}",heart_eyed_cat);
    // 元组 tunple
    let tup: (i32,f64, u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("这个值是：{}",y);
    let x: (i32,f64,u8) = (500,6.4,1);
    let five_hundred = x.0;
    println!("five_hundred的值为{}",five_hundred);
    let six_point_four = x.1;
    println!("six_point_four的值为{}",six_point_four);
    let one = x.2;
    println!("one的值为{}",one);
    // array,rust中的数组是固定长度的
    let arr  = [1,2,3,4,5];
    let mouths = ["January","Febuary","March","April","May","June","July","August","September","October","November","December"];
    // 在方括号中包含每个元素的类型，后跟分号，再后跟元素的数量
    let a: [i32;5] = [1,2,3,4,5];
    // 如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号,然后在方括号中指定数组的长度，
    let a = [3;5]; // 等同于 let a = [3,3,3,3,3]
    // 访问数组元素
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("first={},second={}",first,second);
}