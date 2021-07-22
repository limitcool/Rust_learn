fn main() {
    let mut s = String::from("hello"); // 创建一个可变引用
    change(&mut s);
    println!("{}",s)
}
fn change(some_string: &mut String) { // 接受一个可变引用
    some_string.push_str(",world");
}