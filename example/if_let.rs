fn main() {
    // 全部都是 'Option<i32>'类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion : Option<i32> = None;

    // 'if let'结构读作:若 'let' 将'number'解构成 'Some(i)',则执行语句块
    if let Some(i) = number{
        println!("Matched {:?}",i);
    }
    // 如果要指明失败情形,就用else
    if let Some(i) = letter{
        println!("Matcher is {:?}",i);
    } else {
        // 解构失败。切换到失败情形。
        println!("没有匹配到数字");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;
    if let Some(i) = emotion {
        println!("Matched {:?}",i);
    } else if i_like_letters {
        println!("无法匹配搭配数字");
    }else{
        println!("none")
    };
}