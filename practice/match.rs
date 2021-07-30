#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// fn value_in_cents(coin:Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("状态来自{:?}",(state);
//             25
//         },
//     }
// }
fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
fn main() {
    println!("调用plus_one:{:?}",plus_one(Some(3)));
    // println!("调用value_In_certs:{}",value_in_cents(Coin::Penny));

}