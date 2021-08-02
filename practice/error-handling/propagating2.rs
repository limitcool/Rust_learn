use std::io;
use std::fs::File;
use std::io::Error;

fn read_username_from_file() -> Result<String,io::Error> {
    let mut s = Strinig::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
