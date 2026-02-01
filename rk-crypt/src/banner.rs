use terminal_banner::Banner;
use colored::Colorize;

pub fn print_banner() {
    let banner = Banner::new()
        .text("rk-crypt from RamKit".bold().to_string().into())
        .text(format!("Powered by @{}", "ASX".bold()).into())
        .render();
    println!("{}", banner);
}
