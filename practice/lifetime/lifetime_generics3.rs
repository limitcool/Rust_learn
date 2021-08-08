fn main() {
    let string1 = String::from("我总是第一个");
    let string2 = String::from("我不会被输出");
    let result = returnOne(string1.as_str(),string2.as_str());
    println!("{}",result);
}
fn returnOne<'a>(x:&'a str,y:&str) ->  &'a str{
    x
}
