// 闭包：可以捕获其环境的匿名函数
fn closure_example() {
    let x = 4;
    let equal_to_x = |z| z == x;
    
    println!("闭包比较结果: {}", equal_to_x(4));
}

// 使用迭代器处理元素序列
fn iterator_example() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("迭代值: {}", val);
    }
}

fn main() {
    closure_example();
    iterator_example();
}