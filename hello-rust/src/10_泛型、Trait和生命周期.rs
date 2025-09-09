// 泛型数据类型
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Trait定义共同行为
pub trait Summary {
    fn summarize(&self) -> String;
}

// 生命周期确保引用有效
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("最大的数字是: {}", largest(&number_list));
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    println!("最长的字符串是: {}", longest(string1.as_str(), string2));
}