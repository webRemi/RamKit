////////////////////
// rk-scan by ASX //
////////////////////

use colored::Colorize;
use std::env;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use terminal_banner::Banner;

fn main() {
    let banner = Banner::new()
        .text("rk-scan from RamKit".bold().to_string().into())
        .text(format!("Powered by @{}", "ASX".bold()).into())
        .render();
    println!("{}", banner);

    let args: Vec<String> = env::args().collect();

    let mut custom_port: u32 = 0;

    match args.len() {
        4 => (),
        5 => {
            custom_port = args[4].parse().expect("Invalid port");
        }
        _ => menu(),
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
            menu();
        }
    }
}

fn scan(ip: &str, port: u32) {
    let address = format!("{}:{}", ip, port);
    let server: SocketAddr = address.parse().expect("Unable to parse socket address");
    let duration = Duration::new(2, 0);

    let stream = TcpStream::connect_timeout(&server, duration);
    match stream {
        Ok(_) => println!("[TCP] {}", port),
        Err(_) => (),
    }
}

fn custom_scan(ip: &str, port: u32) {
    scan(ip, port);
}

fn default_scan(ip: &str) {
    let mut handles = vec![];

    let n_threads = 10;
    let n_ports = 100;
    let mut n_scanned: u32 = 1;

    let n_ports_threads = n_ports / n_threads;

    for _thread in 1..=n_threads {
        let ports = match _thread {
            t if t == n_threads => n_scanned..n_ports + 1,
            _ => n_scanned..n_scanned + n_ports_threads,
        };

        let ip_clone = ip.to_string();
        handles.push(thread::spawn(move || {
            for port in ports {
                scan(&ip_clone, port);
            }
        }));
        n_scanned += n_ports_threads;
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn special_scan(ip: &str) {
    let ports = [
        21,   // FTP
        22,   // SSH
        23,   // TELNET
        53,   // DNS
        80,   // HTTP
        81,   // HTTPAPI
        88,   // KERBEROS
        111,  // RPC
        389,  // LDAP
        443,  // HTTPS
        445,  // SMB
        502,  // MODBUS
        636,  // LDAPS
        1433, // MSSQL
        3389, // RDP
        5900, // VNC
        5901, // VNC
        5985, // WINRM
        5986, // WINRMS
        8000, // HTTP
        8080, // HTTP
        8443, // HTTPS
        9001, // PRINT
        9100, // PRINT
    ];

    for port in ports {
        scan(ip, port);
    }
}

fn menu() {
    println!("{} -t <target> [options]", "rk-scan".bold().cyan());
    println!("Options:");
    println!("-d\t\tDefault scan (ports 1-100)");
    println!("-s\t\tSpecial services scan");
    println!("-c\t\tCustom port scan");
    std::process::exit(0);
}
