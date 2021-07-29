pub fn public_function() {
    println!("调用rary库的公共函数public_function()");
}

fn private_function() {
    println!("调用rary库的私有函数private_function()");
}

pub fn indirect_access() {
    println!("调用rary库的间接调用函数'indirect_access()',that\n");
    private_function();
}