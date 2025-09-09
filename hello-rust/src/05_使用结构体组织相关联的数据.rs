// 结构体定义和实例化
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 结构体示例程序
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 方法语法
impl User {
    fn greet(&self) {
        println!("你好，{}！", self.username);
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    let user2 = build_user(
        String::from("another@example.com"), 
        String::from("anotherusername567")
    );
    
    user1.greet();
    user2.greet();
}