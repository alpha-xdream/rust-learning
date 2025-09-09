// 使用Vector储存列表
fn vector_example() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    
    let third: &i32 = &v[2];
    println!("第三个元素是: {}", third);
}

// 使用字符串储存UTF-8编码的文本
fn string_example() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("字符串内容: {}", s);
}

// 使用Hash Map储存键值对
fn hash_map_example() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("Blue队的分数: {:?}", scores.get("Blue"));
}

fn main() {
    vector_example();
    string_example();
    hash_map_example();
}