////////////////////
// rk-exec by ASX //
////////////////////

mod args;
mod scan;
mod utils;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let some_args = args::extract_args(); 

    let hosts = utils::extract_infos(&some_args.ip).await;
    let targets = utils::expand_hosts(hosts);

    let users = match some_args.username {
        Some(ref u) => utils::extract_infos(u).await,
        None => vec![],
    };

    let passwords = match some_args.password {
        Some(ref p) => utils::extract_infos(p).await,
        None => vec![],
    };

    println!("[i] Scanning {} hosts", targets.len());

    let users_arc = Arc::new(users);
    let passwords_arc = Arc::new(passwords);
    let targets_arc = Arc::new(targets);
    let args_arc = Arc::new(some_args);

    scan::attack(users_arc, passwords_arc, targets_arc, args_arc).await;

    Ok(())
}
