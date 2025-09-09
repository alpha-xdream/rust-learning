// 所有权示例
fn ownership() {
    let s = String::from("hello");  // s进入作用域
    takes_ownership(s);             // s的值移动到函数里...
                                   // ... 所以到这里不再有效
    
    let x = 5;                     // x进入作用域
    makes_copy(x);                 // x应该移动到函数里
                                   // 但i32是Copy的，所以后面可以继续使用x
}

fn takes_ownership(some_string: String) { // some_string进入作用域
    println!("{}", some_string);
} // some_string离开作用域并调用`drop`方法

fn makes_copy(some_integer: i32) { // some_integer进入作用域
    println!("{}", some_integer);
} // some_integer离开作用域

// 引用与借用示例
fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("字符串'{}'的长度是{}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    ownership();
    references();
}