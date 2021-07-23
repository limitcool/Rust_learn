fn main() {
    // 通常情况下,'{}' 会被任意变量内容所替换
    // 变量内容会被转换成字符串
    println!("每月有{}天",31);
    // 不加后缀的话，默认为i32类型。
    // 可以添加后缀改变31的类型。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("操作系统有{0}、{1}，{0}天下第一,所以现在我们使用{1}","linux","Windows");

    // 可以使用命名参数
    println!("{body},{heart},{tail}",body="我有身体",heart="有心",tail="没有尾巴");
    // 可以在：后指定特殊的格式。:b将数字转换为二进制
    println!("{}of{:b}人们知道二进制，另一半人不知道",1,2);
    // 可以按照指定宽度来右对齐文本。
    // 下面语句输出"     1",5个空格后面连着1。
    println!("{number:>width$}",number=1,width=6);
    // println! 会检查使用到的参数数量是否正确
    println!("我的名字是{0},{1}{0}","张三","李四");

    // 创建一个包含单个 'i32'的结构体(structure)。命名为'Structure'。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面的语句无法运行
    // println!("这个结构体'{}' 不能输出",Structure(3));
    // 改正^^,注释掉此行
    let pi = 3.141592;
    println!("PI的值是{:.3}",pi);
}