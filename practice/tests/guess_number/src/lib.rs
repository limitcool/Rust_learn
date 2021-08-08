pub struct Guess {
    value:i32,
}

impl Guess{
    pub fn new(value:i32) ->Guess {
        if value<1 ||value >100 {
            panic!("你输入的数字必须在1和100之间,你输入的是{}",value);
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
}