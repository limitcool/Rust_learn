use std::io;
use std::io::Error;
use std::fs::File;

fn read_username_from_file() ->Result<String,io::Error> {
    fs::read_to_string("hello.txt")
}