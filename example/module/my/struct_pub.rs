mod my {
    // 一个公有的结构体。带有一个公有的字段(类型为泛型'T')
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体,带有一个私有的字段(类型为泛型'T')
    #[allow(dead_code)]
    pub struct CloseBox<T> {
        contents: T,
    }
    impl<T> CloseBox<T> {
        // 一个公有的构造体方法
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents: contents }
        }
    }
}

fn main() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox {
        contents: "公有信息",
    };
    // 并且它们的字段可以正常访问
    println!("这个箱子容纳:{}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造/
    // 报错! CloseBox 含有私有字段
    // let close_box = my::closebox { coutents:"分类信息"};

    // 不过带有私有字段的结构体可以使用公有的构造体来创建
    let _closed_box = my::CloseBox::new("分类方法");
}
