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