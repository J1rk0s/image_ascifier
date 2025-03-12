use clap::Parser;

mod ascifier;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub path: String
}

fn main() {
    let args = Args::parse();
}
