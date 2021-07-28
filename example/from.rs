/* From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定了原生类型及其他常见类型的转换功能。

比如，可以很容易地把 str 转换成 String： */
use std::convert::From;
#[derive(Debug)]
struct Number {
    value:i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value:item}
    }
}

fn main() {
    let num = Number::from(30);
    println!("我的数字是{:?}",num);
}