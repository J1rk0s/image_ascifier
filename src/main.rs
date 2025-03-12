use std::{fs::File, io::Write};

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

    let convertor = Ascifier::init(args.image_path, args.charset, args.invert);
    let ascified = convertor.ascify();

    let mut file = File::create(args.save_path).expect("Failed to create file");
    file.write_all(ascified.as_bytes()).expect("Failed to write ascii chars");
}
