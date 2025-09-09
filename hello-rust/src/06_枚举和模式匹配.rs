// 枚举定义
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// match控制流结构
fn match_example() {
    let some_value = Some(3);
    
    match some_value {
        Some(3) => println!("匹配到3"),
        Some(x) => println!("匹配到其他值: {}", x),
        None => println!("没有值"),
    }
}

// if let简洁控制流
fn if_let_example() {
    let some_value = Some(7);
    
    if let Some(7) = some_value {
        println!("匹配到7");
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    match_example();
    if_let_example();
}