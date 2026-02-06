use ipnet::Ipv4Net;
use tokio::fs;

// TAKE FILE IF PROVIDED OTHERWISE USE INFO
pub async fn extract_infos(file_path: &str) -> Vec<String> {
    let mut infos: Vec<String> = Vec::new();
    let contents = fs::read_to_string(file_path).await;
    match contents {
        Ok(content) => {
            for line in content.lines() {
                infos.push(line.to_string());
            }
        },
        Err(_e) => {
            infos.push(file_path.to_string());
        },
    }
    infos
}

// PREPARE TARGETS / PARSE CIDR
pub fn expand_hosts(file_hosts: Vec<String>) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();
    for host in file_hosts {
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
    targets
}