fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{},{}",s1,len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里S离开了作用域，但因为他并不拥有引用值的所有权，所以什么也不会发生