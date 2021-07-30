mod front_on_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}
pub fn eat_at_restaturan() {
    // 绝对路径
    crate::front_on_house::hosting::add_to_waitlist();
    // 相对路径
    front_on_house::hosting::add_to_waitlist();
}

fn server_order() {}
mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::server_order();
    }

    fn cook_order(){}
}