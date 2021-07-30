#![allow(dead_code)]
fn main() {
    let number = 3;
    // 条件必须为布尔值
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number的值为{}", number);
    // loop_func()
    loop_counter();
    // while_number();
    // while_flow();
    // for_flow();
    // for_number_flow();
}
fn else_if(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2")
    }
}
fn loop_func() {
    loop {
        println!("again!")
    }
}
fn loop_counter(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("结果是{}",result);
}
fn while_number() {
    let mut number = 3;
    while number !=0{
        println!("{}!",number);
        number = number -1;
    }
    println!("LIFTOFF!!!")
}
fn while_flow(){
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5{
        println!("现在的值是{}",a[index]);
        index = index + 1;
    }
}
fn for_flow(){
    let a = [10,20,30,40,50];
    for element in a.iter(){
        println!("the value is : {}",element);
    }
}
fn for_number_flow(){
    // 使用rev反转range
    for number in (1..4).rev(){
        println!("{}",number);
    }
    println!("LIFTOFF")
}