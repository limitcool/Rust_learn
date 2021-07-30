#[derive(Debug)]
enum Message {
    Quit, // 没有关联任何数据
    Move { x:i32,y:i32}, // 包含一个匿名结构体
    Write(String), // 包含单独一个字符串
    ChangeColor(i32,i32,i32) // 包含三个i32
}

struct QuitMessage; // 类单元结构体
struct MoveMessage{
    x:i32,y:i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangleColorMessage(i32,i32,i32); // 元组结构体

impl Message {
    fn call(&self) {
        // self.Move.0+self.Move.1
    }
}

fn main() {
    // 实例化
    let test = Message::Write(String::from("hello"));
    test.call()
}