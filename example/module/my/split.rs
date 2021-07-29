// 此声明将会查找名为'my.rs' 或 'my/mod.rs'的文件，并将该文件的内容放到此作用域一个名为'my'的模块里。
mod my;
fn function() {
    println!("调用functiion()");
}

fn main() {
    my::function;
    function();
    my::indirect_access();
    my::nested::function();
    
}