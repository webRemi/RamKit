use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub mode: String,

    #[arg(short, long)]
    pub file: String,

    #[arg(short, long)]
    pub secret: String,
}

pub fn extract_args() -> Args {
    Args::parse()
}
