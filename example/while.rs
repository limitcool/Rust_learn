fn main() {
    // 计数器变量
    let mut n =1;
    // 当'n' 小于 101时循环
    while n <101{
        if n %3==0&& n%5==0{
            println!("FizzBizz")
        }else if n%5==0{
            println!("Bizz")
        }else if n%3==0{
            println!("Fizz")
        }else{
            println!("{}",n)
        }
        // 计数器值增加1
        n+=1
    }
}