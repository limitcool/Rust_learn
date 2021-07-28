enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self,x:i32,y:i32) ->i32 {
        match self {
            Self::Add => x+y,
            self::Subtract => x-y,
        }
    }
}