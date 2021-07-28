/* 在match中,若间接地访问一个变量,则不经过重新绑定就无法在分支中再使用它，match提供了@符号来绑定变量到名称 */
// 'age'函数,返回一个'u32'的值
fn age() -> u32 {
    0
}
fn main() {
    println!("告诉我是你是哪种类型的");
    match age(){
        0 => println!("我想我还没有出生"),
        // 可以直接 'match' 1...12,但怎么把岁数打印出来呢？
        // 相反,在1 ...12 分支中绑定匹配值到 'n',现在年龄就可以读取了。
        n @ 1...12 => println!("我是一个儿童,年龄是:{:?}",n),
        n @13 ... 19 => println!("我是一个青少年,年龄是:{:?}",n),
        n => println!("我是一个老人,年龄是:{:?}",n),
    }
}