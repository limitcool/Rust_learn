// 结构体：struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        email : String::from("Someone@example.com"),
        username : String::from("username"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("youemail@email.com");
    println!("email:{}",user1.email);
}
fn build_user(email:String,username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}