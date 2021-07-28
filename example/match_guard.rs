fn main() {
    let pair = (3,4);
    println!("告诉我一个值:{:?}",pair);
    match pair {
        (x,y) if x == y => println!("这两个数相等"),
        // ^ 'if' 条件部分是一个守卫
        (x,y) if x + y ==0 => println!("这两个数相反"),
        (x,_) if x % 2==1 => println!("第一个数是奇数"),
        _ => println!("没有关联性"),
    } 
}