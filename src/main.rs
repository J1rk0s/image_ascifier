use ascifier::Ascifier;
use clap::Parser;

mod ascifier;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image to ascify
    #[arg(short, long)]
    pub image_path: String,

    /// Ascified image
    #[arg(short, long)]
    pub save_path: String,

    /// Inverts the resulting ascii image
    #[arg(long, default_value_t = false)]
    pub invert: bool,

    /// Custom charset
    #[arg(long, default_value = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ")]
    pub charset: String
}

fn main() {
    let args = Args::parse();

    let convertor = Ascifier::init(args.image_path, args.save_path, args.charset, args.invert);
    convertor.ascify();
}
