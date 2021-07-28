fn main() {
    // 将'optional' 设为 'option<i32>'类型
    let mut optional =Some(0);
    // 这读作:当 'let' 将 'optional' 解构成 'Some(i)' 时，就
    // 执行语句块 ('{}'),否则就 'break'
    while let Some(i) = optional {
        if i>9 {
            println!("Greater than 9,quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'Try again.",i);
            optional = Some(i+1)
        }
    }
}