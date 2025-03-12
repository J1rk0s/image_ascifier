use std::{fs::File, io::Write};

use rusty_imager::{filters::Grayscale, formats::ImageFormat, image::Image};

use crate::utils;

pub struct Ascifier {
    image_path: String,
    save_path: String,
    charset: String,
    invert: bool
}

impl Ascifier {
    pub fn init(image_path: String, save_path: String, charset: String, invert: bool) -> Self {
        Self { image_path, save_path, charset, invert }
    }

    pub fn ascify(&self) {
        let mut img = Image::from_file(&self.image_path).expect("File not supported");
        img.apply_filter(Grayscale::new());

        let ascified = self.process(&img);

        let mut file = File::create(&self.save_path).expect("Failed to create file");
        file.write_all(ascified.as_bytes()).expect("Failed to write ascii chars");
    }

    fn process(&self, img: &Image) -> String {
        let mut res: String = String::new();

        for i in (0..img.get_width()).rev() {
            for j in (0..img.get_height()).rev() {
                if let Some(color) = img.get_pixel(j, i) {
                    let val: u8;

                    if self.invert {
                        val = color.invert().r;
                    } else {
                        val = color.r;
                    }

                    let index = utils::remap(val as isize, 0, 255, (self.charset.len() - 1) as isize, 0) as usize;

                    let c = self.charset.chars().nth(index).expect("Invalid charset index");

                    res.push(c);
                }
            }

            res.push('\n');
        }

        res
    }
}