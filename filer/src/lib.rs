use std::collections::HashMap;
use std::fs;
use std::io::{self};
use console::style; // 导入console库，用于颜色输出

pub struct Config {
    pub path: String, // 要搜索的目录路径
    pub filter: Vec<String>, // 要过滤的文件类型，如".txt"
}

fn is_filter(path: &str, filter: &Vec<String>) -> bool {
    for f in filter {
        if path.ends_with(f) {
            return true;
        }
    }
    false
}

/// 递归遍历目录，记录文件名和路径
/// 参数:
///   - dir_path: 当前遍历的目录路径
///   - name_count: 文件名到路径列表的映射
/// 返回值:
///   - io::Result<()>: 操作结果
fn walk_dir(
    dir_path: &str,
    filter: &Vec<String>,
    name_count: &mut HashMap<String, Vec<String>>,
) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir_path) {
        // 遍历目录中的每一项
        for entry in entries {
            let entry = entry?;
            let path_buf = entry.path();
            let is_dir = path_buf.is_dir();
            let path = path_buf.to_str().unwrap();

            // 跳过隐藏文件和目录
            if path.starts_with(".") {
                continue;
            }

            if !is_dir && is_filter(&path, &filter) {
                if let Some(file_name_with_ext) = entry.file_name().to_str() {
                    // 获取无扩展名的文件名
                    let file_name = std::path::Path::new(file_name_with_ext).file_stem().and_then(|s| s.to_str()).unwrap_or(file_name_with_ext);
                    // 获取或创建该文件名的路径列表
                    // 如果文件名已存在，则添加到对应列表
                    // 如果文件名不存在，则创建新列表
                    let paths = name_count
                        .entry(file_name.to_string())
                        .or_insert(Vec::new());
                    paths.push(path.to_string());
                }
            }

            // 如果是目录，则递归遍历
            if is_dir {
                walk_dir(path, filter, name_count)?;
            }
        }
    }
    Ok(())
}


/// 查找并打印指定目录下所有重名的文件和文件夹
/// 参数:
///   - path: 要搜索的目录路径
/// 返回值:
///   - io::Result<()>: 操作结果，成功返回Ok(()), 失败返回Err
pub fn run(config: Config) -> io::Result<()> {
    let path = config.path.as_str();
    // 使用HashMap记录文件名和对应的路径列表
    // key: 文件名, value: 包含该文件名的所有路径
    let mut name_count: HashMap<String, Vec<String>> = HashMap::new();

    walk_dir(path, &config.filter, &mut name_count)?;

    let mut total_files = 0; // 记录总文件数
    let mut _total_folders = 0; // 记录总文件夹数
    let mut has_duplicates = false; // 标记是否有重名的文件和文件夹
    let mut keys = name_count.keys().collect::<Vec<&String>>();
    keys.sort();
    for duplicate_name in keys {
        let paths = name_count.get(duplicate_name).unwrap();
        if paths.len() <= 1 {
            continue;
        }
        if !has_duplicates {
            println!("{}", style("重名的文件和文件夹:").red().bold());
            has_duplicates = true;
        }
        total_files += paths.len();

        println!("{} #{}", style(duplicate_name).cyan(), style(paths.len().to_string()).red().bold());
        for path_str in paths {
            let path = std::path::Path::new(&path_str);
            let parent = path.parent().and_then(|s| s.to_str()).unwrap_or("");
            let styled_name = style(duplicate_name).red().bold().to_string();
            let ext = path.extension().and_then(|ext| ext.to_str()).map(|ext_str| format!(".{}", ext_str)).unwrap_or_default();
            let result = format!("{}{}{}{}", parent, std::path::MAIN_SEPARATOR, styled_name, ext);
            println!("{}", result);
        }
    }
    if !has_duplicates {
        println!("{}", style("没有重名的文件和文件夹。").green().bold());
    }else{
        println!("共找到重名的文件和文件夹:{}", style(total_files).red().bold());
    }

    Ok(())
}
