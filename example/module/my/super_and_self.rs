// 可以在路径中使用super(父级)和self(自身)关键字,从而在访问项中消除歧义，以及防止不必要的路径硬编码。
fn function() {
    println!("调用'function()'");
}

fn another_function() {
    println!("调用anther_function()");
}

mod cool {
    pub fn function() {
        println!("调用cool::function()");
    }
}

mod my {
    fn function() {
        println!("调用my::function()");
    }
    mod cool {
        pub fn function() {
            println!("调用'my::cool::fucntion()'");
        }
    }
    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为'function'的函数
        println!("调用'my::indirect_call()'");
        // 'self'关键字表示当前模块作用域-在这个例子中是my
        //调用self::function() 和调用'function'都得到相同的结果
        self::function();
        function();

        //我们也可以使用'self'来访问'my'内部的另一个模块:
        self::cool::function;
        // 'super' 关键字表示父作用域('在my 模块外面')
        super::function();
        super::another_function();
        // 这将在 *crate* 作用域内绑定'cool::function()'。
        // 在这个例子中,create 作用域是最外面的作用域
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
