// 将'deeply::nested::function'路径绑定到'other_function'。
use deeply::nested::function as other_function;
fn function() {
    println!("调用function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("调用deeply::nested::function()");
        }
    }
}

fn main() {
    //  更容易访问 'deeply::nested::function'
    other_function();
    println!("进入块");
    {
        // 这和'use deeply::nested::function as function'等价。
        // 此 'function'将掩蔽外部的同名函数
        use deeply::nested::function;
        function();

        // 'use'绑定拥有局部作用域，在这个例子中,'function()'的掩蔽只存在这个代码块中
        println!("离开代码块")
    }
    function();
}
