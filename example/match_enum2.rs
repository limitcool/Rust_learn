// 将'optional' 定为 'Option<i32>' 类型
fn main() {
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("{:?}",i);
    },
    _ => {},
};
}