// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]
fn main() {
    let decimal = 65.4321_f32;
    // 错误 不提供隐式类型转换
    // let integer: u8 =decimal;
    // 可以显示转换
    let integer = decimal as u8;
    let character = integer as char;
    
    println!("Casting: {} -> {} -> {}",decimal,integer,character);

    // 可以把任何类型转换为无符号类型T时，会不断加上或减去 (std::T::MAX+1)
    // 直到值位于新类型T的范围内。

    // 1000 已经在 u16的范围内
    println!("1000 as a u 16 is: {}",1000 as u16);

    // 1000-256-256-256 =232
    // 事实上的处理方式是:从最低有效位(LSB, least significant bits) 开始保留8位，
    // 然后剩余位置，直到最高有效位(MSB,most significant bit) 都被抛弃。
    // MSB就是二进制的最高位，LSB就是二进制的最低位，按日常的书写习惯就是最左边一位和最右边一位。
    println!("1000 as a u8 is : {}",1000 as u8);
    // -1+256 =255
    println!(" -1 as a u8 is :{}",(-1i8) as u8);
    // 对正数,这就和取模一样
    println!("1000 mod 256 is :{}",1000%256);

    // 当转换到有符号类型时，(位操作的)结果就和"先转换到对应的无符号类型，如果MSB是1则该值为负"是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!("128 as a i16 is :{}",128 as i16);
    println!("128 as a i8 is :{}",128 as i8);
    println!("1000 as a u8 is :{}",1000 as u8);
    println!("232 as a i8 is:{}",232 as i8);
}