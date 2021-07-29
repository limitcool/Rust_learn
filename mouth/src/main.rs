use std::io;
fn main() {
    loop{
    println!("请输入月份：");
    let mut input= String::new();
    io::stdin().read_line(&mut input)
        .expect("读取失败");
    let arr =["May","may"];
    match input.as_str(){
    if input ==arr[0] => println!("11"),
    "Thu" | "thu" => println!("other"),
    _ => println!("1"),    
    }
}
}