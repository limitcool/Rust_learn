/* for in 结构能以几种方式与iterator互动，如果没有特别指定，for循环会对给出的集合应用into_iter函数,把它转换成一个迭代器。
这并不是把集合变成迭代器的唯一方法,其他的方法有iter和iter_mut函数 */
// iter在每次迭代中借用集合中的一个元素。这样集合本身不会被改变, 循环之后仍可以使用。
fn main() {
    // Vec 定义了一个动态增长的数组
    let names = vec!["Bob","Frank","Ferris"];
    
    // iter-在每次迭代中借用集合中的一个元素。这样集合本身不会改变，循环之后仍可以使用。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("我们中间有个锈迹斑斑的人"),
            _ => println!("你好{}",name),
        }
    }
}