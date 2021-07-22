fn first_word(s: &String) -> usize {
    // 使用as_bytes()将String转换为字节数组，
    let bytes = s.as_bytes();
    // 使用iter方法在字节数组上创建一个迭代器
    // iter方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回
    // i是索引，元组中的&item是单个字节
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn main(){
    let mut s = String::from("hello world");
    let word = first_word(&s); //
    println!("{}",word);
    s.clear(); //清空字符串，使其等于 “ ”
}