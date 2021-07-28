// 可以使用绑定来 [结构] enum变体,例如Option:
fn some_number() -> Option<u32> {
    Some(42)
}
fn main() {
    match some_number() {
        // 得到 'Some'的值，如果它的值匹配,就与n绑定
        Some(n @ 42) => println!("答案是:{}!",n),
        // 匹配另一个数字
        Some(n) => println!("另一个数字:{}",n),
        _ => (),
    }
}