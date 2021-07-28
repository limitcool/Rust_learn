fn main() {
    let number = 13;
    println!(":{}",number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5|7|11 => println!("这是一个素数"),
        // 匹配一个闭区间范围
        13...19 => println!("十位数"),
        // 处理其他情况
        _ => println!("一个特殊的数")
    }
    
    let boolean = true;
    // match也是一个表达式
    let binary = match boolean {
        // match分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    
    };
    println!("{} -> {}",boolean,binary);
}