// 一个名为 'my'的模块
mod my {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("调用my::private_function()");
    }
    // 使用'pub'修饰符来改变默认可见性
    pub fn function() {
        println!("调用'my::function()'");
    }
    // 在同一个模块中,项可以访问其他项,即使它是私有的。
    pub fn indirect_access() {
        print!("调用 'my::indirect_access()',这里换行\n");
        private_function();
    }
    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("调用'mymod::nested::function()'");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("调用'my::nested::private_function()'");
        }
        // 使用 'pub(in path)' 语法定义的函数只在给定的路径中可见。
        // 'path' 必须是父模块(parent module) 或祖先模块(ancestor module)
        pub(in my) fn public_function_in_my() {
            print!("调用'my::nested::public_function_in_my(),在这里换行\n'");
        }
        // 使用'pub(self)' 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("调用my::nested::public_function_in_nested");
        }
        // 使用'pub(super)' 语法定义的函数只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("调用my::nested::public_function_in_super_mod()");
        }
    }
    pub fn call_public_function_in_my(){
        print!("调用'my::call_public_function_in_my()',在这里换行\n");
        nested::public_function_in_my();
        println!(">");
        nested::public_function_in_super_mod();
    }

    // 'pub(crate)' 使得函数只在当前create中可见
    pub(crate) fn public_function_in_create() {
        println!("调用mymod::public_function_in_create()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function(){
            println!("调用'my::private_nested::function()'");
        }
    }
}
fn function() {
    println!("调用'function()'");
}

fn main() {
    function();
    my::function();
    // 公有项,包括嵌套模块内的，都可以在 父模块外部访问。
    my::indirect_access();
    my::nested::function();
    my::call_public_function_in_my();

    // pub(crate) 项可以在同一个carte中的任何地方访问
    my::public_function_in_create();

}