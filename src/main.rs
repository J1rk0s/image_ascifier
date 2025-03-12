use ascifier::Ascifier;
use clap::Parser;

mod ascifier;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image to ascify
    #[arg(short, long)]
    pub image_path: String,

    /// Ascified image
    #[arg(short, long)]
    pub save_path: String
}

fn main() {
    let args = Args::parse();

    Ascifier::ascify(args.image_path, args.save_path);
}
