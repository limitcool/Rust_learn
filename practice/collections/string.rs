fn main() {
    // 新建一个空的String
    let mut s = String::new();
    let data = "初始化内容";
    let s = data.to_string();

    // 该方法也可以直接用于字面值
    let f = "再次初始化内容".to_string();
    println!("s:{}",s);
    println!("f:{}",f);
    let n = String::from("第三次初始化内容,存储至n");
    println!("n:{}",n);
    let hello = String::from("السلام عليكم");
    println!("{}",hello);
    let hello = String::from("Dobrý den");
    println!("{}",hello);
    let hello = String::from("Hello");
    println!("{}",hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}",hello);
    let hello = String::from("नमस्ते");
    println!("{}",hello);
    let hello = String::from("こんにちは");
    println!("{}",hello);
    let hello = String::from("안녕하세요");
    println!("{}",hello);
    let hello = String::from("你好");
    println!("{}",hello);
    let hello = String::from("Olá");
    println!("{}",hello);
    let hello = String::from("Здравствуйте");
    println!("{}",hello);
    let mut hello = String::from("Hola");
    println!("{}",hello);
    hello.push_str("Rust");
    println!(",{}",hello);
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}",s1);
    println!("{}",s2);
    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}",s3);
    let s1 = String::from("tic");
    let s2 = String::from("toc");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",s);
    let s1 = String::from("hello");
    let h = &s1[0..4];
    println!("{}",h);
    for c in "测试一下".chars() {
        println!("{}",c);
    }
    for b in "再测试一下".bytes() {
        println!("{}",b);
    }
}