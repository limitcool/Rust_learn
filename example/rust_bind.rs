fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将'an_integet' 复制到'copied_integer'
    let copied_integer = an_integer;

    println!("一个数字 :{}",copied_integer);
    println!("一个布尔值:{}",a_boolean);
    println!("单元值:{:#?}",unit);

    // 编译器会对未使用的变量绑定产生警告：可以给变量加上下划线前缀来清除警告
    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;
}