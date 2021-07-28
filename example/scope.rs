fn main() {
    // 此绑定生存于 main函数中
    let long_lived_binding = 1;
    // 这是一个代码块，比main函数拥有更小的作用域
    {
        // 此绑定域只存在于本代码块
        let short_lived_binding = 2;
        println!("内部短；{}",short_lived_binding);
        // 此绑定掩蔽看外面的绑定
        let long_lived_binding = 5_f32;
        println!("内部长；{}",long_lived_binding);
    }
    // 代码块结束
    // println!("外部短:{}",short_lived_binding);
    println!("外部长：{}",long_lived_binding);
    let long_lived_binding = 'a';
    println!("外部长:{}",long_lived_binding);
}