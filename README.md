# Rust_learn
# 安装VSC插件
安装Native Debug
# 所有权
1.Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
2.值在任一时刻有且只有一个所有者。
3.当所有者（变量）离开作用域，这个值将被丢弃。

# 函数
.push_str() 在字符串后追加字面值
.clone() 克隆数据
.len() 返回字符串的长度
.as_bytes() 将String转化为字节数组
.iter() 在字节数组上创建一个迭代器，iter方法返回集合中的每一个元素，
.enumerate() enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。

# Structs(结构体)

:b将数字转换为二进制

# std::fmt包含多种traits来控制文字显示，其中重要的两个trait的基本形式如下：
fmt::Debug:使用{:?}标记，格式化文本以供调试使用
fmt::Display: 使用{}标记。以更优雅和友好的风格来格式化文本

// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]
// size_of_val 函数 返回一个变量所占的字节数
std::mem::size_of_val() 

# VSC
跳转行数:Ctrl +G

# for in iter
.into_iter()    //for in默认使用函数 如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器。
    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 // “移除”（move）了。
.iter() //-在每次迭代中借用集合中的一个元素。这样集合本身不会改变，循环之后仍可以使用。
.iter_mut() // 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。

# 指针和引用
## 对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，和 C 那样的语言用法不一样。

### 解引用使用 *
### 解构使用 &、ref、和 ref mut

 // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;