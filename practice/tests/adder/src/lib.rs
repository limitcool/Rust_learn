#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    fn another() {
        panic!("让测试停止");
    }
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height < other.height
        }
    }

    // #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width:8,height:7};
        let smaller = Rectangle {width:5,height:1};
        assert!(larger.can_hold(&smaller));
    }
    // #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{width:8,height:7};
        let smaller = Rectangle{width:5,height:1};
        assert!(smaller.can_hold(&larger))
    }
    pub fn add_two(a:i32) ->i32 {
        a +2
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(4,add_two(2));
    }

    pub fn greeting(name:&str) -> String{
        String::from("Hello!")
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Hello");
        assert!(result.contains("Hello"));
    }
}
