fn main(){
    factorial(5)
}

fn factorial(n:i32){
    if n == 1{
        return ;
    }else{
        println!("n={}",n);
        return factorial(n*(n-1));
    }
}