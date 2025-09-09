// 如何编写测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("让这个测试失败");
    }
}

// 控制测试如何运行
fn main() {
    println!("运行测试: cargo test");
}