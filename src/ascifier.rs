use std::{fs::File, io::Write};

use rusty_imager::{filters::Grayscale, formats::ImageFormat, image::Image};

use crate::utils;

const CHARSET: &'static str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct Ascifier {}

impl Ascifier {
    pub fn ascify(image_path: String, save_path: String) {
        let mut img = Image::from_file(&image_path).expect("File not supported");
        img.apply_filter(Grayscale::new());

        let ascified = Ascifier::process(&img);

        let mut file = File::create(save_path).expect("Failed to create file");
        file.write_all(ascified.as_bytes()).expect("Failed to write ascii chars");
    }

    fn process(img: &Image) -> String {
        let mut res: String = String::new();

        for i in (0..img.get_width()).rev() {
            for j in (0..img.get_height()).rev() {
                if let Some(color) = img.get_pixel(j, i) {
                    let index = utils::remap(color.invert().r as isize, 0, 255, (CHARSET.len() - 1) as isize, 0) as usize;

                    let c = CHARSET.chars().nth(index).expect("Invalid charset index");

                    res.push(c);
                }
            }

            res.push('\n');
        }

        res
    }
}