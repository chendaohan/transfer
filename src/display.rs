use std::path::Path;

// 显示文件或文件夹的路径，大小和已经传输了多少数据
pub fn display_files_info(path: &Path, total_length: &str, transferred_length: usize) {
    println!("path: {}", path.display());
    println!(
        "file length: {}, transferred length: {}, total length: {}",
        display_length_in_appropriate_units(path.metadata().unwrap().len() as usize),
        display_length_in_appropriate_units(transferred_length),
        total_length
    );
    println!("==============================================================================");
}

// 以合适的单位显示长度
pub fn display_length_in_appropriate_units(length: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut unit_index = 0_usize;
    let mut display_length = length;
    while display_length > 1024 {
        display_length /= 1024;
        unit_index += 1;
    }
    format!(
        "{:.2} {}",
        length as f64 / 1024_i32.pow(unit_index as u32) as f64,
        units[unit_index]
    )
}

#[test]
fn test() {
    display_files_info(
        Path::new("target"),
        &display_length_in_appropriate_units(20 * 1024 * 1024 + 3 * 1024 + 473),
        5 * 1024 + 543,
    );
}
