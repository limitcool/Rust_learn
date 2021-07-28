/* 在处理嵌套循环的时候可以break或continue外层循环。在这种情况下，循环必须用一些'label'标签来注明,并且标签必须传递给break/continue语句 */
#![allow(unreachable_code)]

fn main() {
    'outer : loop {
        println!("进入循环外部");
        '_inner : loop {
            println!("进入循环内部");

            // 这里'break'之是中断内部的循环
            // break;

            // 这会中断外部循环
            break 'outer;
        }
        println!("这里的内容不会被打印");
    }
    println!("退出外部循环！")
}