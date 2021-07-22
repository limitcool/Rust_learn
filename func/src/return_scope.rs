fn main() {
    let s1 = gives_ownership();  // gives_owenship 将返回值，移给S1
    let s2 = String::from("hello"); // S2 进入作用域
    let s3 = takes_and_gives_back(s2); // S2被移动到takes_and_gives中，它也将返回值移动给s3
} //这里,s3移出作用域并被丢弃，s2也移除作用域，但已被移走,所以什么也不会发生，s1移出作用域并被丢弃

fn gives_ownership() ->String {  // gives_ownership将返回值移动给调用它的函数
    let some_string = String::from("gives_ownership_hello"); // some_string进入作用域
    some_string //返回 some_string 并移除给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string:String)-> String{ // a_string 进入作用域
    a_string // 返回 a_string并移出给调用的函数
}