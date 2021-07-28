fn main() {
    let n = 5;

    if n < 0 {
        println!("{}是负数",n);    
    }else if n>0 {
        println!("{}是正数",n);
    }else{
        println!("{}是零",n);
    }
    let big_n = 
        if n< 10 && n>-1 {
            println!("n是一个小数，n乘以10");
            10 * n
        } else {
            println!(",n是一个大数,减少一半");
            n /2
        };
    println!("{}->{}",n,big_n);
}