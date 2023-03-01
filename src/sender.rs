use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
    path::{Path, PathBuf},
};

use crate::{display, DEFAULT_PORT, BUFFER_LENGTH};

// 发送文件或文件夹
pub fn send(args: &[&str]) {
    // 获取的ip和端口号
    let socket_addr = match args[0] {
        "-i" | "--ip-port" => args[1].parse::<SocketAddrV4>().unwrap(),
        "--ip" => SocketAddrV4::new(args[1].parse::<Ipv4Addr>().unwrap(), DEFAULT_PORT),
        _ => panic!("缺少关键参数"),
    };

    let mut stream = TcpStream::connect(socket_addr).unwrap();
    let path = Path::new(args[2]);
    let mut buffer = vec![0_u8; BUFFER_LENGTH];
    let path_parent = path.parent().unwrap();
    if path.is_file() {
        let length = path.metadata().unwrap().len() as usize;
        stream.write_all(format!("{length}\0").as_bytes()).unwrap();
        send_file(&mut stream, &mut buffer, path, path_parent);
        let total_length = display::display_length_in_appropriate_units(length);
        display::display_files_info(path, &total_length, length);
    } else {
        let mut paths = VecDeque::new();
        traverse_dir(Path::new(args[2]), &mut paths);
        paths.insert(0, path.to_path_buf());

        // 计算和发送总长度
        let total_length: usize = paths
            .iter()
            .map(|p| {
                if p.is_file() {
                    p.metadata().unwrap().len() as usize
                } else {
                    0_usize
                }
            })
            .sum();
        stream
            .write_all(format!("{total_length}\0").as_bytes())
            .unwrap();
        let total_length = display::display_length_in_appropriate_units(total_length);
        let mut transferred_length = 0;
        paths.iter().for_each(|p| {
            if p.is_file() {
                send_file(&mut stream, &mut buffer, p, path_parent);
                transferred_length += p.metadata().unwrap().len() as usize;
            } else {
                send_dir(&mut stream, p, path_parent);
            }
            display::display_files_info(p, &total_length, transferred_length);
        });
    }
}

// 递归遍历文件
fn traverse_dir(dir_path: &Path, subpaths: &mut VecDeque<PathBuf>) {
    for dir_entry in dir_path.read_dir().unwrap() {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            traverse_dir(&path, subpaths);
        }
        subpaths.insert(0, path);
    }
}

// 发送文件夹
fn send_dir(stream: &mut TcpStream, path: &Path, path_parent: &Path) {
    stream
        .write_all(format!("dir:{}\0", path.strip_prefix(path_parent).unwrap().display()).as_bytes())
        .unwrap();
}

// 发送文件
fn send_file(stream: &mut TcpStream, buffer: &mut [u8], path: &Path, path_parent: &Path) {
    let file_description = format!(
        "file:{}:{}\0",
        path.strip_prefix(path_parent).unwrap().display(),
        path.metadata().unwrap().len()
    );
    stream.write_all(file_description.as_bytes()).unwrap();
    let mut file = File::open(path).unwrap();
    while let Ok(length) = file.read(buffer) {
        stream.write_all(&buffer[..length]).unwrap();

        if length < buffer.len() {
            break;
        }
    }
}
