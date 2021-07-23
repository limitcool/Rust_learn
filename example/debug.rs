// 这个结构体不能使用 'fmt::Display' 或'fmt::Debug' 来进行打印
struct UnPrintable(i32);
// 'derive'属性会自动创建所需的实现，使这个'struct' 能使用 'fmt::Debug' 打印
struct DebugPrintbale(i32);
#[derive(Debug)]
// 所有的std类天生都可以用{:?}来打印。
fn main(){

}