use std::fmt; // 使用use导入 fmt 模块，使'fmt::Display'可用

// 带有两个数字的结构体。推导出'Debug',以便于 'Display'的输出比较

#[derive(Debug)]
struct MinMax(i64,i64);

// 实现 'MinMax'的 'Display'
impl fmt::Display for MinMax {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        //使用 self.number 来表示各个数据。
        write!(f,"({},{})",self.0,self.1)
    }
}

// 为了比较 定义一个含有字段的结构体

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 'Point2D' 实现 'Display'
impl fmt::Display for Point2D {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 'x'和'y'的值
        write!(f,"x:{},y:{}",self.x,self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}+{}i",self.real,self.imag)
    }
}
fn main() {
    let minmax = MinMax(0,14);
    println!("对比结构体:");
    println!("显示:{}",minmax);
    println!("Debug:{:?}",minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("这个最大随机数是{big},最小随机是{small}",small=small_range,big=big_range);
    let point = Point2D {x:3.3,y:7.2};
    println!("对比端点：");
    println!("显示:{}",point);
    println!("Debug:{:?}",point);

    let complex = Complex {real:3.3,imag:7.2};
    println!("对比Complex");
    println!("显示:{}",complex);
    println!("Debug:{:?}",complex);
}