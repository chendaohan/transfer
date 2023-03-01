// 显示帮助信息
pub fn help() {
    println!("在局域网中传输文件或文件夹。\n");
    println!("Usage:\n> transfer\n");
    println!(
        "Subcommands:\n\n\
            transfer sender <argument> <path> - 发送文件或文件夹。\n\
            -i | --ip-port <ip:port> 或 --ip <ip>: 默认端口号是8000。\n\n\
            transfer receiver <argument> <path> - 接收文件或文件夹。\n\
            (optional) -p | --port <port>: 默认端口号是8000.\n"
    );
}
