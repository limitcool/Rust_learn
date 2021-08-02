#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("bule")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(5),
    ];
    for i in &row {
        println!("{:?}",i);
    }
}