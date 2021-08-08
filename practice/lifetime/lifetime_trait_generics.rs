use std::fmt::Display;
fn longest_with_annoucement<'a,T>(x:&'a str,y:&'a str,ann:T) -> &'a str where T:Display
{
    println!("Annoucement!{}",ann);
    if x.len() >y.len(){
        x
    }else{
        y
    }
}
fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("worldd!");
    let result = longest_with_annoucement(string1.as_str(),string2.as_str(), "返回最长的值");
    println!("{}",result);
}