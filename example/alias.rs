// 'NanoSecond' 是'u64' 的新名字
type NanoSecond = u64;
type Inch = u64;

// 通过这个属性屏蔽警告
#[allow(non_camel_case_types)]

type u64_t = u64;
fn main() {
    // 'NanoSecond' = 'Inch' = 'u64_t' = 'u64'.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches:Inch = 2 as u64_t;

    // 注意类型别名并不能提供额外的类型安全，因为别名并不是新的类型
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds+ inches)
}