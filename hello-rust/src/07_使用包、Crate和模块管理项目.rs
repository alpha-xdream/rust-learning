// 模块示例
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
    }
}

// 使用use关键字
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}