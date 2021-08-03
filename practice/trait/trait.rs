pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(阅读更多,作者:{})",self.summarize_author())
    }
}

pub struct Tweet {
    content:String,
    username:String,
    reply:bool,
    retweet:bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("破坏新闻!{}",item.summarize());
}


fn main() {
    let tweet = Tweet {content:String::from("随便写写"),username:String::from("Jams"),reply:false,
        retweet:false,};
    println!("{}",tweet.summarize());
    notify(tweet);
}