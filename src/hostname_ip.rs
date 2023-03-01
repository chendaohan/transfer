use std::{env::consts, net::Ipv4Addr, process::Command, str::FromStr};

// 获取本机局域网 ip
pub fn get_lan_ip() -> Ipv4Addr {
    match consts::OS {
        "windows" => {
            let output = get_output_in("ipconfig");
            let (output, _, _) = encoding_rs::GBK.decode(&output);
            get_windows_lan_ip(&output)
        }
        "linux" => {
            let output = get_output_in("ifconfig");
            let output = String::from_utf8(output).expect("decode utf-8 failed!");
            get_linux_lan_ip(&output)
        }
        _ => panic!("This os does not support!"),
    }
}

// 调用系统内置程序返回输出
fn get_output_in(name: &str) -> Vec<u8> {
    Command::new(name).output().expect("command error!").stdout
}

// 解析 ipconfig 输出，获得 ip
fn get_windows_lan_ip(output: &str) -> Ipv4Addr {
    output
        .lines()
        .map(|l| l.trim_end())
        .filter_map(|l| {
            if l.contains("IPv4 地址") {
                l.find(": ").map(|i| &l[i + 2..])
            } else {
                None
            }
        })
        .filter_map(|l| Ipv4Addr::from_str(l).ok())
        .next()
        .expect("lan ip resolution failed!")
}

// 解析 ifconfig 输出，获得 ip
fn get_linux_lan_ip(output: &str) -> Ipv4Addr {
    output
        .lines()
        .map(|l| l.trim_start())
        .filter_map(|l| l.strip_prefix("inet "))
        .filter_map(|l| l.find(' ').map(|x| &l[0..x]))
        .filter_map(|l| Ipv4Addr::from_str(l).ok())
        .find(|a| !a.is_loopback())
        .unwrap()
}
