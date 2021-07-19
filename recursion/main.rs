fn main(){
    let mut res = factorial(5);
    println!("res={}",res);
}

fn factorial(&n:i32){
    if n == 1{
        return ;
    }else{
        return factorial(n*(n-1));
    }
}