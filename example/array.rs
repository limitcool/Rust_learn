use std::mem;

// 此函数借用一个slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}",slice[0]);
    println!("the slice has {} elements",slice.len());
}

fn main() {
    // 定长数组 (类型标记是多余的)
    let xs: [i32;5] = [1,2,3,4,5];
    // 所有元素可以初始化成相同的值

    let ys: [i32;500] = [1;500];

    // 下标从0开始
    println!("数组xs中的第一个元素是:{}",xs[0]);
    println!("数组xs中的第二个元素是:{}",xs[1]);

    // 'len'返回数组的长度
    println!("数组xs的长度是:{}",xs.len());

    // 数组是在栈中分配的
    println!("数组xs占用的字节是:{}",mem::size_of_val(&xs))

    // 数组可以自动被借用成为slice
    println!("借用整个数组作为一个切片");
    analyze_slice(&xs);
}