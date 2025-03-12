use rusty_imager::{formats::ImageFormat, image::Image};

const CHARSET: &'static str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct Ascifier {}

impl Ascifier {
    pub fn ascify(image_path: String, save_path: String) {
        let img = Image::from_file(&image_path).expect("File not supported");

        let ascified = Ascifier::process(&img);
    }

    fn process(img: &Image) -> String {
        let mut res: String = String::new();

        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(color) = img.get_pixel(i, j) {
                    
                }
            }

            res.push('\n');
        }

        todo!()
    }
}