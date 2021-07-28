#![allow(dead_code)]
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNubmers {
    Add,
    Subtract,
}

// 新建一个类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNubmers;

fn main() {
    // 我们可以通过它的别名来引用每个变量
    let x = Operations::Add;
    println!("{:?}",x);
}