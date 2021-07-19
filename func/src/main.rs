fn main() {
    println!("Hello, world!");
    more_function(12, 13);
    another_function();
    let x = five();
    let x = plus_one(x);
    println!("five（）返回值为{}",x)
}
fn another_function() {
    println!("另一个函数!");
    second_function(50);
}
// x类型指定为i32
fn second_function(x:i32){
    println!("X的值是:{}",x);
}
// 函数有多个参数的时候用逗号分隔
fn more_function(x:i32,y:i32) {
    println!("x的值为{},y的值为{}",x,y);
}
// 表达式的结尾没有分号，如果加上了分号就会变为语句

fn five() -> i32 {
    5
}
fn plus_one(x:i32) -> i32 {
    x + 1
}