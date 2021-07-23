use std::io;
fn main() {
    loop{
            let mouth_arr = ["Jan","s"];
    println!("请输入月份：");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input)
        .expect("读取失败");
    println!("{}",input);
    if input == mouth_arr[0]{
        println!("1月");
    }else if input == "feb" {
        println!("2月");
    }
    }
}