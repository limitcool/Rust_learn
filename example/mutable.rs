fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("改变之前:{}",mutable_binding);
    mutable_binding +=1;
    println!("改变之后:{}",mutable_binding);

    // _immutable_binding +=1;
}