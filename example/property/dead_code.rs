fn used_function() {
    println!("调用'used_function()函数'")
}

// #[allow(dead_code)] 属性可以禁用 'dead_code' lint
#[allow(dead_code)]
fn unused_function() {

}

#[allow(dead_code)]
fn noisy_unused_function() {
    
}
fn main() {
    used_function();
}