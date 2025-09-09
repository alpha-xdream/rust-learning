// 用panic!处理不可恢复的错误
fn panic_example() {
    panic!("这是一个不可恢复的错误");
}

// 用Result处理可恢复的错误
fn result_example() -> Result<i32, String> {
    let x = 5;
    if x > 3 {
        Ok(x)
    } else {
        Err(String::from("x太小了"))
    }
}

fn main() {
    // panic_example(); // 取消注释会触发panic
    
    match result_example() {
        Ok(val) => println!("成功获取值: {}", val),
        Err(e) => println!("错误: {}", e),
    }
}