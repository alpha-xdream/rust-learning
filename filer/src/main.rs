use std::io::{self, Write};

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    // 提示用户输入目录路径
    print!("请输入目录路径: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let path = input.trim();
    println!("您输入的目录路径是: {}", path);

    filer::run(path)?;

    Ok(())
}
