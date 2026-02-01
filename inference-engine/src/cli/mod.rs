use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "inference-engine")]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(short, long)]
    pub interactive: bool,
}

pub fn run() {
    let _cli = Cli::parse();
    unimplemented!()
}
