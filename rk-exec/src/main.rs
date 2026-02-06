////////////////////
// rk-exec by ASX //
////////////////////

mod args;
mod scan;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let some_args = args::extract_args(); 

    let hosts = utils::extract_infos(&some_args.ip).await;
    let targets = utils::expand_hosts(hosts);

    let users = utils::extract_infos(&some_args.username.as_deref().unwrap_or("")).await;
    let passwords = utils::extract_infos(&some_args.password.as_deref().unwrap_or("")).await;

    println!("[i] Scanning {} hosts", targets.len());

    scan::attack(&users, &passwords, &targets, &some_args).await;

    Ok(())
}