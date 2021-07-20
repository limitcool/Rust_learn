fn factorial(n:i8) -> i8{
    if n == 1{
        return 1;
    }
    factorial(n-1)*n
}
fn main() {
    println!("{}",factorial(3));
}

