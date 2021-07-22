fn main() {
    let s = String::from("hello"); //S进入作用域
    takes_ownership(s); // s的值移动到函数里，所以在这里不在有效
    let x = 5; // x进入作用域
    makes_copy(x); // x 应该移动函数里
    println!("主程序内的Interge{}", x); // 但i32是copy的，所以后面可继续使用x
    
}
fn takes_ownership(some_string: String) {
    println!("函数里面的string{}", some_string);
}
fn makes_copy(some_interge: i32) {
    println!("函数里面的Interge{}",some_interge);
}
