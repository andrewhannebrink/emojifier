use crate::mosaic;
use image::{ImageBuffer, RgbImage};
use crate::path;
use std::fs;
static DIMENSIONS: (u32, u32) = (1920, 1080);

    fn wipe_zoom_dir() {
        fs::remove_dir_all(path::ZOOM_OUTPUT_DIR);
        fs::create_dir(path::ZOOM_OUTPUT_DIR);
    }

    pub fn all_lil_imgs_img(lil_imgs_dir: &str) {
        wipe_zoom_dir();
        let mut image: RgbImage = ImageBuffer::new(DIMENSIONS.0, DIMENSIONS.1);
        let lil_imgs = mosaic::get_lil_imgs_from_dir(&lil_imgs_dir.to_string(), 5);
        // set a central pixel to white
        
        // Makes a white image
        for x in 0..DIMENSIONS.0 {
            for y in 0..DIMENSIONS.1 {
                *image.get_pixel_mut(x, y) = image::Rgb([255,255,255]);
            }
        }
        let n = lil_imgs.len() as f32;
        let x = DIMENSIONS.0 as f32;
        let y = DIMENSIONS.1 as f32;
        let px = (n * x / y).sqrt().ceil();
        let py = (n * y / x).sqrt().ceil();

        let mut sx: f32;
        let mut sy: f32;
        if (px*y/x)*px < n {
            sx = y / (px*y/x).ceil();
        } else {
            sx = x/px;
        }
        if (py*x/y)*py < n {
            sy = x / (x*py/y).ceil();
        } else {
            sy = y/py;
        }

        println!("{}, {}, {}, {}", sx, sy, px, py);
        // write it out to a file
        image.save(&path::zoom_output_path(&"0001".to_string())).unwrap();
    }

