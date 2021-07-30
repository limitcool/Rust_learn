#[derive(Debug)]
enum IpAddrKind {
    Ipv4(String),
    Ipv6(String),
}
#[derive(Debug)]
enum IpAddr {
    Ipv4(u8,u8,u8,u8),
    Ipv6(String),
}
fn main() {
    let home = IpAddrKind::Ipv4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::Ipv6(String::from("::1"));
    println!("打印home:{:#?}",home);
    println!("打印loopback:{:#?}",loopback);
    let home = IpAddr::Ipv4(127,0,0,1);
    let loopback = IpAddr::Ipv6(String::from("::1"));
    println!("隐蔽home再次打印{:#?}",home);
    println!("隐蔽loopback再次打印{:#?}",loopback);
}