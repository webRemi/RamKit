use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub connect: Option<String>,

    #[arg(short, long)]
    pub ip: String,

    #[arg(short, long)]
    pub username: Option<String>,

    #[arg(short, long)]
    pub password: Option<String>,
}

pub fn extract_args() -> Args {
    Args::parse()
}
