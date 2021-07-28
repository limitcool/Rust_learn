/* for in结构可以遍历一个iterator (迭代器)。创建迭代器的一个最简单的方法就是使用区间标记a..b。
这会生成从a(包含此值)到b(不含此值)的，步长为1的一系列值。 */

fn main() {
    // 'n' 将在每次迭代中分别取1,2,...,100
    for n in 1..101{
    // 使用a..=b可以将两端都包含在内
        if n % 15 == 0{
            println!("FizzBuzz");
        }else if n%3 ==0 {
            println!("Fizz");
        }else if n%5 == 0{
            println!("Buzz");
        }else{
            println!("{}",n);
        }
    }
}