use std::io;

fn main() {
    loop{
    println!("请输入小写月份简称：");
    let mut input= String::new();
    io::stdin().read_line(&mut input)
        .expect("读取失败");
    let command = input.trim();
    // let arr =["May","may"];
    // match input.as_str(){
    // if input ==arr[0] => println!("11"),
    // "Thu" | "thu" => println!("other"),
    // _ => println!("1"),    
    // }

    // println!("你输入的是{}",& command);
    match command {
        "jan"|"Jan" => println!("1月"),
        "feb"|"Feb" => println!("2月"),
        "mar"|"Mar" => println!("3月"),
        "apr"|"Apr" => println!("4月"),
        "may"|"May" => println!("5月"),
        "jun"|"Jun" => println!("6月"),
        "jul"|"Jul" => println!("7月"),
        "aug"|"Aug" => println!("8月"),
        "sep"|"Sep" => println!("9月"),
        "oct"|"Oct" => println!("10月"),
        "nov"|"Nov" => println!("11月"),
        "dec"|"Dec" => println!("12月"),
        "exit"|"quit" => {
            println!("你输了Exit,正在进行退出");
            break}
        _ => println!("输入不正确,请检查输入,或输入exit退出"),
    }
}
}