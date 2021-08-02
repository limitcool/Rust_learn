use std::cmp::PartialOrd
fn largest<T>(list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if &item >
         largest{
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1,3,4,50,23,535,100];
    let result = largest(&number_list);
    println!("数字总最大的数是{}",result);
    let char_list = vec!['a','b','c'];
    let result = largest(&char_list);
    println!("字母中最大的数是{}",result);
}