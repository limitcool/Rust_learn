use std::collections::HashMap;
fn main() {
    let mut scores= HashMap::new(); 
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    println!("{:?}",scores);

    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_score = vec![10,50];
    let scores:HashMap<_,_> = teams.iter().zip(initial_score.iter()).collect();

    let field_name = String::from("喜欢的颜色");
    let field_value = String::from("蓝色");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    println!("{:?}",map);
    let team_name = String::from("Blue");
    let score =scores.get(&team_name);
    // println!("{:?}",score)
    for (key,value) in &scores {
        println!("{}:{}",key,value);
    }
    let mut scores = HashMap::new();
    scores.insert(String::from("红色"),100);
    scores.insert(String::from("红色"),150);
    let team_name = String::from("红色");
    // scores.get(&team_name);
    println!("{:?}",scores.get(&team_name));
    // 我们经常会检查某个特定的键是否有值，如果没有就插入一个值，为此哈希map有一个特有的API，叫做entry，
    // 它获取我们想要检查的键作为参数。entry函数的返回值是一个枚举，Entry,它代表了可能存在也可能不存在的值，
    // 比如我们想要检查黄队的键是否关联了一个值，如果没有就插入50，对于蓝队也是如此
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    println!("{:?}",scores);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}",scores);
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        // *解引用
        *count +=1;
    }
    println!("{:?}",map);
}
