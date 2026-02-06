use smb::{Client, UncPath};
use std::str::FromStr;
use std::net::TcpStream;
use std::time::Duration;
use std::net::SocketAddr;

// CONNECT TO SHARE
pub async fn connect_share(client: &Client, server: &str, share_name: &str, username: &str, password: &str) -> Result<(), smb::Error>{
    let target_path = UncPath::from_str(&format!(r"\\{}\{}", server, share_name)).unwrap();
    client.share_connect(&target_path, &username, password.to_string()).await
}

// LIST AVAILABLE SHARES
pub async fn list_shares(connection: Result<(), smb::Error>, client: &Client, server: &String) {
   match connection {
        Ok(_c) => {
            println!("[SHARES] Enumerating shares...");
            match client.list_shares(server).await {
                Ok(shares) => {
                    println!("\tShares:\n\t=======");
                    for share in shares {
                        match share.netname.as_ref() {
                            Some(ob) => {
                                let share_name = String::from_utf16(&ob.data).unwrap();
                                println!("\t{}\t{}", server, share_name);
                            }
                            None => (),
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

// CHECK IF SMB IS OPEN ON TARGET
pub async fn check_open(ip: &str, port: u32) -> bool {
    let address = format!("{}:{}", ip, port);
    let server: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    let duration = Duration::new(2, 0);

    let stream = TcpStream::connect_timeout(&server, duration);
    match stream {
        Ok(_s) => {
            true
        }
        Err(_) => {
            false
        }
    }
}