fn main() {
    // 新建一个动态数组
    let v = vec![100,32,56];
    // 遍历数组中的所有元素并打印
    for i in &v {
        println!("{}",i)
    }
    let mut v1 = vec![23,245,242];
    for i in &mut v1 {
        // 解引用运算符*获取i中的值
        *i+=50;
        println!("将数组中的每个值增加50:{}",i)
    } 
}