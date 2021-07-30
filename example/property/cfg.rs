// 这个函数仅当目标系统是Linux的时候才会编译
#[cfg(target_os = "windows")]
fn are_you_on_win() {
    println!("you are running windows!")
}

#[cfg(target_os="linux")]
fn are_you_on_linux() {
    println!("you are running linux")
}

fn main() {
    are_you_on_win();
    println!("你确定吗？");
    if cfg!(target_os="windows") {
        println!("是的，你运行的是win");
    }else {
        println!("经过进一步检查,你运行的不是win");
    }        
}