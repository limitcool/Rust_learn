struct Point<T,U> {
    x:T,
    y:U,
}

fn main() {
    let both_integer = Point{x:5,y:10};
    let both_float = Point{x:0.5,y:0.1};
    let integer_float = Point{x:0.35,y:50};
    let integet_char = Point{x:0.355,y:'s'};
    let string_char= Point{x:"sfsfs",y:'s'};
}