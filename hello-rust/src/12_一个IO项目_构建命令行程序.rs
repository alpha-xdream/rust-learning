use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析参数时出错: {}", err);
        std::process::exit(1);
    });
    
    println!("搜索: {}", config.query);
    println!("在文件中: {}", config.filename);
    
    run(config);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("读取文件失败");
    
    println!("文件内容:\n{}", contents);
}