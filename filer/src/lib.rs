use std::collections::HashMap;
use std::fs;
use std::io::{self};
use console::style; // 导入console库，用于颜色输出

/// 递归遍历目录，记录文件名和路径
/// 参数:
///   - dir_path: 当前遍历的目录路径
///   - name_count: 文件名到路径列表的映射
///   - duplicates: 存储重复文件路径的列表
/// 返回值:
///   - io::Result<()>: 操作结果
fn walk_dir(
    dir_path: &str,
    name_count: &mut HashMap<String, Vec<String>>,
    duplicates: &mut Vec<String>,
) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir_path) {
        // 遍历目录中的每一项
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = entry.file_name().to_str() {
                // 获取或创建该文件名的路径列表
                // 如果文件名已存在，则添加到对应列表
                // 如果文件名不存在，则创建新列表
                let paths = name_count
                    .entry(file_name.to_string())
                    .or_insert(Vec::new());
                paths.push(path.to_str().unwrap().to_string());
                if paths.len() > 1 {
                    duplicates.extend(paths.clone());
                }
            }

            // 如果是目录，则递归遍历
            if path.is_dir() {
                walk_dir(path.to_str().unwrap(), name_count, duplicates)?;
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
pub fn run(path: &str) -> io::Result<()> {
    // 使用HashMap记录文件名和对应的路径列表
    // key: 文件名, value: 包含该文件名的所有路径
    let mut name_count: HashMap<String, Vec<String>> = HashMap::new();

    // 存储所有重复文件的路径
    let mut duplicates: Vec<String> = Vec::new();

    walk_dir(path, &mut name_count, &mut duplicates)?;

    // 如果有重复文件，则打印所有重复路径
    if !duplicates.is_empty() {
        use console::style;
        println!("{}", style("重名的文件和文件夹:").red().bold());
        for path in duplicates {
            println!("{}", style(path).yellow());
        }
    } else {
        println!("{}", style("没有重名的文件和文件夹。").green().bold());
    }

    Ok(())
}
