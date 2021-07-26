use std::fmt::{self,Formatter,Display};
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32,bool)) -> (bool,i32) {
    // 可以使用 'let' 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    (boolean,integer)
}

struct Matrix(f32,f32,f32,f32);
impl Display for Matrix {
    fn fmt(&self,f: &mut Formatter) -> fmt::Result {
        writeln!(f,"({} {})",self.0,self.1);
        writeln!(f,"({} {})",self.2,self.3);
    }
}

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8,2u16,3u32,4u64,-1i8,-2i16,-3i32,-4i64,0.1f32,0.2f64,'a',true);
    // 通过元组的下标来访问具体的值
    println!("长元组第一个值是:{}",long_tuple.0);
    println!("长元组第二个值是:{}",long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8,2u16,2u32),(4u64,-1i8),-2i16);

    // 元组可以打印
    println!("tuple_of_tuples:{:?}",tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("too long tuple:{:?}",too_long_tuple);
    
    // 元组可以被解构，从而将值绑定给变量
    let tuple = (1,"hello",4.5,true);
    let (a,b,c,d) =  tuple;
    println!("{:?},{:?},{:?},{:?}",a,b,c,d);
    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("{}",matrix)
}