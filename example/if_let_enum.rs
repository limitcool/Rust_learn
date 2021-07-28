enum Foo {
    Bar,
    Baz,
    Qux(i32)
}

fn main() {
    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c =Foo::Qux(100);

    // 变量a匹配到了Foo::bar
    if let Foo::Bar = a{
        println!("A is Foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar,因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    // 变量C 匹配到了 Foo::Quc，它带有一个值,就和上面例子中的Some()类似。
    if let Foo::Qux(value) = c{
        println!("c is {}",value);
    }
}