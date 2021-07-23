// 推导 'Structure' 的 'fmt:Debug' 实现
// 'Structure' 是一个包含单个'i32'的结构体
#[derive(Debug)]
struct  Structure(i32);

// 将 'Structure' 放到结构体 'Deep' 中，然后使'Deep'也能够打印。
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    //  使用 '{:?}' 打印和使用 '{}'类似
    println!("一年有{:?}个月",12);
    println!("{1:?}, {0:?} 是某个{actor:?}名字",
        "李四",
        "张三",
        actor = "演员的");
    // 'Structure' 也可以打印! 
    println!("现在{:?}将要打印输出 ",Structure(3));
    // 使用 'derive'的一个问题是不能控制输出的形式
    // 假如我只想展示一个7怎么办
    println!("现在{:?}将要打印输出",Deep(Structure(7)));
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:?}",peter);
}
