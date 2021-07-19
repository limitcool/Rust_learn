// 引入io(输入/输出)库到当前作用域
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn 声明一个新函数，{作为函数体的开始，()表示没有参数
fn main() {
    println!("猜一个数字!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("秘密数字是{}", secret_number);
    loop {
        println!("请输入你要猜测的数字");
        // 创建一个储存用户输入的变量，rust变量默认不可变，前缀mut可变
        // let foo = 5; 不可变
        // let mut foo = 5;可变
        // ::语法表明new是String类型的一个关联函数，关联函数是针对类型实现的，在这个例子中是string而不是string的某个特定实例，一些语言把它称为静态方法
        // new函数创建一个新的空字符串
        let mut guess = String::new();
        // 调用io库中的函数stdin，向read_line传递参数,&表示这个参数是一个引用，他运行多处代码访问同一数据
        io::stdin().read_line(&mut guess)
            .expect("读取失败");
        // 创建一个新值，隐藏以前的guess，使用trim去除空白字符
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
        }
    }
}
