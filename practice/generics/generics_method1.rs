struct Point{
    x:f32,
    y:f32,
}

impl Point {
    fn s(&self) ->f32 {

        // powi是求平方
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}

fn main() {
    let test = Point{x:10.0,y:5.0};
    println!("{}",test.s());
}
