fn is_odd(n:u32) -> bool {
    n % 2 ==1
}

fn main() {
    println!("求1000以下所有奇数的和");
    let upper = 1000;
    // 命令式的写法
    // 声明累加器变量
    let mut acc =0;
    // 迭代:0,1,2,....无穷大
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        }else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("命令式:{}",acc);

    // 函数式的写法
    let sum_of_squard_odd_numbers: u32 = 
        (0..).map(|n| n*n) //所有自然数取平方
            .take_while(|&n| n<upper) // 取小于上限的
            .filter(|&n| is_odd(n)) // 取奇数
            .fold(0,|sum,i|sum+i);
    println!("functional Style:{}",sum_of_squard_odd_numbers);
}