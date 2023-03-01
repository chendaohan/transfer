use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddrV4, TcpListener},
    path::Path, env::consts,
};

use crate::{display, hostname_ip, BUFFER_LENGTH, DEFAULT_PORT};

// 接收文件和文件夹
pub fn receive(args: &[&str]) {
    let port;
    let receive_dir;
    match args[0] {
        "-p" | "--port" => {
            port = args[1].parse::<u16>().unwrap();
            receive_dir = Path::new(args[2]);
        }
        _ => {
            port = DEFAULT_PORT;
            receive_dir = Path::new(args[0]);
        }
    }

    let ip = hostname_ip::get_lan_ip();
    let addr = SocketAddrV4::new(ip, port);
    receive_file(addr, receive_dir);
}

// 接收文件或文件夹
fn receive_file(addr: SocketAddrV4, receive_dir: &Path) {
    let listener = TcpListener::bind(addr).unwrap();
    let (stream, _) = listener.accept().unwrap();
    let mut stream = BufReader::new(stream);
    let mut buffer = vec![0_u8; BUFFER_LENGTH];
    // 读取总长度
    let mut description = Vec::new();
    let length = stream.read_until(b'\0', &mut description).unwrap();
    let total_length = String::from_utf8_lossy(&description[..length - 1])
            .parse::<usize>()
            .unwrap();
    let total_length = display::display_length_in_appropriate_units(total_length);
    let mut transferred_length = 0;
    // 通过循环接收文件并写入硬盘
    loop {
        description.clear();
        let length = stream.read_until(b'\0', &mut description).unwrap();
        if description.is_empty() {
            break;
        }
        let description = String::from_utf8_lossy(&description[..length - 1]);
        let mut description = description.split(':');
        // 数据的类型标识
        let type_id = description.next().unwrap();
        // 接收文件夹的路径
        let mut path = receive_dir.to_path_buf();
        let concat_path = description.next().unwrap();
        let concat_path = if consts::OS == "windows" {
            concat_path.replace('/', "\\")
        } else {
            concat_path.replace('\\', "/")
        };
        path.push(concat_path);
        // 对不同的数据类型做不同处理
        if type_id == "file" {
            let file_length = description.next().unwrap().parse::<usize>().unwrap();
            let mut file = File::create(&path).unwrap();
            // 能填满缓冲区的个数
            let loop_count = file_length / BUFFER_LENGTH;
            // 不能填满缓冲区的剩余长度
            let last_length = file_length % BUFFER_LENGTH;
            for _ in 0..loop_count {
                stream.read_exact(&mut buffer).unwrap();
                file.write_all(&buffer).unwrap();
            }
            stream.read_exact(&mut buffer[..last_length]).unwrap();
            file.write_all(&buffer[..last_length]).unwrap();
            // 计算已经传输的数据量
            transferred_length += file_length;
        } else {
            fs::create_dir(&path).unwrap();
        }
        display::display_files_info(&path, &total_length, transferred_length);
    }
}
