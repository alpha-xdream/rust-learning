// 采用发布配置自定义构建
#[cfg(debug_assertions)]
fn debug_build() {
    println!("这是调试构建");
}

#[cfg(not(debug_assertions))]
fn release_build() {
    println!("这是发布构建");
}

// 将crate发布到Crates.io
// 需要先在Cargo.toml中添加metadata
// [package]
// name = "my_crate"
// version = "0.1.0"
// authors = ["Your Name <you@example.com>"]
// description = "A sample crate"
// license = "MIT"

fn main() {
    debug_build();
    release_build();
}