// 该函数将闭包作为参数并调用它。
fn apply<F>(f:F) where
    // 闭包没有输入值和返回值
    F: FnOnce(){
        f();
}

// 输入闭包，返回一个'i32'整型的整数
fn apply_to_3<F>(f:F) -> i32 where
    // 闭包处理一个'i32'整型并返回一个'i32'整型
    F: Fn(i32) -> i32 {
        f(3)
}

fn main() {
    use std::mem;
    let greeting = "hello";
    // 不可复制的类型。
    // 'to_owned'从借用的数据创建所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量:通过引用捕获'greeting',通过值捕获'farewall'
    let diary = || {
        // 'greeting通过引用捕获，故而需要的闭包是'Fn''。
        println!("我说{}",greeting);

        // 下文改变了 'farewall',因而要求闭包通过可变引用来捕获它。
        // 现在需要'FnMut'。
        farewell.push_str("!!!");
        println!("Then I screamed {}.",farewell);
        println!("Now I can Sleep.zzzz");

        // 手动调用 drop 又要求闭包通过值获取 'farewell'。
        // 现在需要 'FnOnce'。
        mem::drop(farewell);
    };
    // 以闭包作为参数，调用函数 'apply'。
    apply(diary);

    // 闭包'double' 满足 'apply_to_3'的trait约束。
    let double = |x| 2 * x;
    println!("3 doubled:{}",apply_to_3(double));
}