/// 重要摘录
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
/// novel:小说
    let novel = String::from("123.很久很久以前");
    let first_sentence = novel.split('.')
        .next()
        .expect("找不到'.'");
    let i =ImportantExcerpt {part:first_sentence};
    println!("{:?}",i)
}