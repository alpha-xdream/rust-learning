use std::io::{self, Write};

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    // 提示用户输入目录路径
    print!("请输入目录路径: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let path = input.trim();

    print!("请输入检测的文件类型(用空格分隔，如.txt .md): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let filter: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    filer::run(filer::Config{
        path: path.to_string(),
        filter: filter,
    })?;

    Ok(())
}
