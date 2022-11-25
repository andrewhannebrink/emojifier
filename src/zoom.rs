use crate::mosaic;
use image::{ImageBuffer, RgbImage};
use crate::path;
use std::fs;

//  fn wipe_zoom_dir() {
//      fs::remove_dir_all(path::ZOOM_OUTPUT_DIR);
//      fs::create_dir(path::ZOOM_OUTPUT_DIR);
//  }

//  pub fn all_lil_imgs_img(lil_imgs_dir: &str) {
//      wipe_zoom_dir();
//      let mut image: RgbImage = ImageBuffer::new(1920, 1080);
//      let lil_imgs = mosaic::get_lil_imgs_from_dir(&lil_imgs_dir.to_string(), 5);
//      // set a central pixel to white
//      *image.get_pixel_mut(5, 5) = image::Rgb([255,255,255]);
//      // write it out to a file
//      image.save(&path::zoom_output_path(&"0001".to_string())).unwrap();
//  }

