/* 对指针来说，解构和解引用要区分开,因为这两者的概念是不同的,和C那样的语言用法不一样，
解引用使用*，解构使用&,ref,和 ref mut*/
fn main() {
    // 获得一个'i32'类型的引用,'&'表示引用。
    let reference = &4;
    match reference {
        // 如果用'&val'这个模式去匹配'reference'，就相当于做这样的比较:
        // '&i32'(即'reference'的类型)
        // '&val'(即用于匹配的模式)
        // ^ 如果去掉匹配的'&','i32'应当赋给'val'。
        // 因此可用'val'表示被'reference'引用的值4
        &val => println!("通过解构得到一个值:{:?}",val),
    }

    // 如果不想用 '&',需要在匹配前解引用。
    match *reference {
        val => println!("通过解引用得到一个值:{:?}",val),
    }

    // 如果一开始就不用引用,会怎样？ 'reference' 是一个'&' 类型,因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用,因为右边不是。
    let _not_a_reference = 3;
    // Rust 对这种情况提供了'ref'。它更改了赋值行为,从而可以对具体值创建引用。
    // 下面这行将得到一个引用
    let ref _is_a_reference = 3;
    // 相应地，定义两个非引用的变量，通过'ref'和'ref mut'仍可以取的引用。
    let value =5;
    let mut mut_value = 6;

    // 使用'ref'关键字来创建引用
    // 下面的 r是'&i32'类型，它像'i32'一样可以直接打印,因此用法上似乎看不出什么区别。
    // 前面例子中的 'println!'里就不能是'*val',因为不能对整数解引用
    match value {
        ref r => println!("得到了一个值的引用:{:?}",r),
    }

    // 类似的使用 'ref mut'
    match mut_value {
        ref mut m => {
            // 已经获得了 'mut_value'的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("m的值添加10,'mut_value':{:?}",m);
        },
    }
}