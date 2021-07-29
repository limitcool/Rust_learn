// 这是个crate 是一个库文件
#![crate_type = "lib"]
// 库的名称是"rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("调用rary库的'public_function()函数'");
}
fn private_function() {
    println!("调用rary库的'private_function()函数'");
}
pub fn indirect_access() {
    print!("间接调用rary库的'indirect_access()函数',在这里换行\n");
    private_function();
}