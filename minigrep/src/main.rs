use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}


fn main() {
    let args :Vec<String> = env::args().collect();
    let config = match(Config::new(&args)) {
        Ok(cfg) => cfg,
        Err(msg) => {
            println!("Error:{}",msg);
            return;
        }
    };
    println!("搜索的内容是{}",config.query);
    println!("进行搜索的文件是{}",config.filename);
    let contents = fs::read_to_string(config.filename)
        .expect("打开文件失败"); 
    println!("文件的内容\n{}",contents);
}


// 构造关联函数
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}