////////////////////
// rk-scan by ASX //
////////////////////

use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut custom_port: u32 = 0;

    match args.len() {
        4 => (),
        5 => {
            custom_port = args[4].parse().expect("Invalid port");
        }
        _ => { 
            println!("{} -t <ip> -<mode>", &args[0]);
            return
        }
    }

    let ip = &args[2];
    let mode = &args[3];

    match mode.as_str() {
        "-d" => default_scan(ip),
        "-s" => special_scan(ip),
        "-c" => {
            custom_scan(ip, custom_port);
        } 
        _ => { 
            println!("{} -t <ip> -<mode>", &args[0]);
            return
        }
    }
}

fn scan(ip: &str, port: u32) {
    let address = format!("{}:{}", ip, port);
    let stream = TcpStream::connect(address);
    match stream {
        Ok(_) => println!("[TCP] {}", port),
        Err(_) => (),
    }
}

fn custom_scan(ip: &str, port: u32) {
    scan(ip, port);
}

fn default_scan(ip: &str) {
    for port in 1..=100 {
        scan(ip, port);
    }
}

fn special_scan(ip: &str) {
    let ports = [
        21,     // FTP
        22,     // SSH
        23,     // TELNET
        53,     // DNS
        80,     // HTTP
        81,     // HTTPAPI
        88,     // KERBEROS
        111,    // RPC
        389,    // LDAP
        443,    // HTTPS
        445,    // SMB
        502,    // MODBUS
        636,    // LDAPS
        1433,   // MSSQL
        3389,   // RDP
        5900,   // VNC
        5901,   // VNC
        5985,   // WINRM
        5986,   // WINRMS
        8000,   // HTTP
        8080,   // HTTP
        8443,   // HTTPS
        9001,   // PRINT
        9100    // PRINT
    ];
    
    for port in ports {
        scan(ip, port);
    }

}

