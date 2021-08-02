use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("打开文件失败");
}