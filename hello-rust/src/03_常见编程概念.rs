// 变量与可变性
fn variables() {
    let mut x = 5;
    println!("x的值为: {}", x);
    x = 6;
    println!("x的新值为: {}", x);
}

// 数据类型
fn data_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);
}

// 函数
fn another_function(x: i32) {
    println!("另一个函数的参数x值为: {}", x);
}

fn main() {
    variables();
    data_types();
    another_function(5);
}