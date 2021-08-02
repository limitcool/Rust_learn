fn largest_i32(list:&[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list:&[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn small_char(list:&[char]) ->char {
    let mut small = list[0];
    for &item in list.iter() {
        if item < small {
            small = item;
        }
    }
    small
}
fn main() {
    let number_list = [1,2,3,153,23424,53435];
    let mut largest =largest_i32(&number_list);
    println!("数字最大的值是{}",largest);
    let char_list = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n',
        'o','p','q','r','s','t','u','v','w','x','y','z'];
    let result = largest_char(&char_list);
    println!("字母中最大的值是{}",result);
    let result = small_char(&char_list);
    println!("字母中最小的值是{}",result);
}