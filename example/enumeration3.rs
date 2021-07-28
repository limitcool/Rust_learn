// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式地 'use' 'Status' 各个名称使他们直接可用,而不需要指定他们来自'Status'
    use Status::{Rich,Poor};
    // 自动地 'use' 'Work'内部的各个名称
    use Work::*;
    // 'Poor' 等价于 'Status::Poor'
    let status= Poor;
    let work = Civilian;
    match status {
        // 注意这里没有用完整路径，因为上面显式的使用了'use'.Civilian
        Rich => println!("富人有很多钱"),
        Poor => println!("穷人没有钱"),
    }

    match work {
        // 不使用完整路径
        Civilian => println!("文员工作"),
        Soldier => println!("军人战斗"),
    }
}