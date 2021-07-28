fn main() {
    // 带后缀的字面量,其类型在初始化时就已经知道了。
    let x = 1u8; // u8占用1个字节
    let y = 2u32; // u32占用4个字节
    let z = 3f32; //  f32占用4个字节
    // 无后缀的字面量,其类型取决于如何使用她们。
    let i =1; // i32 占用4个字节
    let f =1.0; // f64 占用8

    // size_of_val 返回一个变量所占的字节数
    println!("size of 'x' in bytes:{}",std::mem::size_of_val(&x));
    println!("size of 'y' in bytes:{}",std::mem::size_of_val(&y));
    println!("size of 'z' in bytes:{}",std::mem::size_of_val(&z));
    println!("size of 'i' in bytes:{}",std::mem::size_of_val(&i));
    println!("size of 'f' in bytes:{}",std::mem::size_of_val(&f));
}