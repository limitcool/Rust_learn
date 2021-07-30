struct A; // 具体类型 'A'。
struct S(A) // 具体类型 'S'。
struct Sgen<T>(T) // 泛型类型 'Sgen'

// 下面全部函数都得到了变量的所有权,并立即使之离开作用域,将变量释放

// 定义一个函数 'reg_fn',接受一个'S'类型的参数'_s'
// 因为没有'<T>'这样的类型泛型参数,所以这不是泛型函数
fn reg_fn(_s:S) {}

// 定义一个函数'gen_spec_t',接受一个'Sgen<A>'类型的参数'_s'。
// 'Sgen<>'显式的接受了类型参数'A',且在'gen_spec_t'中,'A'没有被用作
// 泛型类型参数,所以函数不是泛型的。



fn gen_spec_t(_s:Sgen<A>) {}

// 定义一个函数 'gen_spec_i32',接受一个'Sgen<i32>' 类型的参数'_s'
// 'Sgen<>' 显式地接受了类型参量'i32'，而'i32'是一个具体类型
// 由于'i32'不是一个泛型类型,所以这个函数也不是泛型的
fn gen_spec_i32(_s:Sgen<i32>) {}

// 定义一个函数'generic',接受一个'Sgen<T>'类型参数的'_s'。
// 因为'Sgen<T>' 之前有'<T>',所以这个函数是关于'T'的泛型函数

fn generic(_s:Sgen<T>){}

fn main() {
    // 使用非泛型函数   
    reg_fn(S(A)); //具体类型
    gen_spec_t(Sgen(A)); // 隐式地指定类型参数'A'
    gen_spec_i32(Sgen(6)); //隐式地指定类型参数'i32'

    // 为 'generic()' 显式地指定类型参数'char'
    generic::<char>(SGen('a'));
    // 为 'generic()' 隐式地指定类型参数'char'
    generic(Sgen('c'));
}
