fn main() {
    let string1 = String::from("sdfsfsfsf");
    {
        let string2 = String::from("hjkhnkjjdhfdf");   
        let result = longest(string1.as_str(),string2.as_str());
    }
    
    println!("较大的是{}",result);
}
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}