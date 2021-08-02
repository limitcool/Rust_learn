pub struct Guess {
    value:i32,
}

impl Guess{
    pub fn new(value:i32) -> Guess {
        if value <1 || value >100 {
            panic!("猜测的值必须在1和100之间,{}"，value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}