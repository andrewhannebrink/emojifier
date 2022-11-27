use crate::mosaic;
use image::{ImageBuffer, RgbaImage, DynamicImage};
use image::GenericImageView;
use image::imageops::FilterType;
use image::imageops::replace;
use crate::path;
use std::fs;
static DIMENSIONS: (u32, u32) = (1920, 1080);

#[derive(Debug)]
struct ZoomImageInfo {
  img: DynamicImage,
  zoom_coords: (i32, i32),
  depth: u32
}

fn wipe_zoom_dir() {
    fs::remove_dir_all(path::ZOOM_OUTPUT_DIR);
    fs::create_dir(path::ZOOM_OUTPUT_DIR);
}

fn plain_white_img() -> RgbaImage {
    let mut canvas_img: RgbaImage = ImageBuffer::new(DIMENSIONS.0, DIMENSIONS.1);
    for x in 0..DIMENSIONS.0 {
        for y in 0..DIMENSIONS.1 {
            *canvas_img.get_pixel_mut(x, y) = image::Rgba([255,255,255, 0]);
        }
    }
    canvas_img
}

fn all_lil_imgs_img(lil_imgs_dir: &str) -> Vec<ZoomImageInfo>{
    wipe_zoom_dir();
    let mut canvas_img: RgbaImage = plain_white_img();
    let mut lil_imgs = mosaic::get_lil_imgs_from_dir(&lil_imgs_dir.to_string(), 5);
    let mut zoom_imgs: Vec<ZoomImageInfo> = Vec::new();
    // set a central pixel to white
    
    // Makes a white image
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

    let mut i = 0;
    for x in 0..px as i64 {
        for y in 0..py as i64 {
            if i >= lil_imgs.len() {
                break
            }
            zoom_imgs.push(ZoomImageInfo {
                img: lil_imgs[i].img.clone(), //TODO satisfying the borrow checker here is hard
                zoom_coords: ((x*(sx as i64)) as i32, (y*(sy as i64)) as i32),
                depth: sx as u32
            });
            let temp_img = zoom_imgs[i].img.resize(
                sx as u32, sy as u32, FilterType::Gaussian);
            replace(&mut canvas_img, &temp_img, x*(sx as i64), y*(sy as i64)); 
            i = i + 1;
        }
    }

    println!("{}, {}, {}, {}", sx, sy, px, py);
    // write it out to a file
    canvas_img.save(&path::zoom_output_path(&"00001".to_string())).unwrap();
    zoom_imgs
}

pub fn zoom(lil_imgs_dir: &str) {
    let mut zoom_imgs = all_lil_imgs_img(lil_imgs_dir);
    let mut canvas_img: RgbaImage = plain_white_img();
    let (z, b, d) = (4.0, 0_f32, 0_f32);
    for i in 0..zoom_imgs.len() {
        dbg!(zoom_imgs[i].zoom_coords);
        let x = zoom_imgs[i].zoom_coords.0 as f32;
        let y = zoom_imgs[i].zoom_coords.1 as f32;
        let new_x = (z * x + b) as i32;
        let new_y = (z * y + d) as i32;
        dbg!(new_x, new_y);
        let prev_size = zoom_imgs[i].depth;
        let new_size = (z * prev_size as f32).floor() as u32;
        zoom_imgs[i].zoom_coords = (new_x, new_y);
        zoom_imgs[i].depth = new_size;

        // if x or y is out of bounds do nothing
        if new_y >= 0 && y.floor() as u32 + new_size <= canvas_img.dimensions().1 {
            if new_x >= 0 && x.floor() as u32 + new_size <= canvas_img.dimensions().0 {
                let temp_img = zoom_imgs[i].img.resize(
                    new_size, new_size, FilterType::Gaussian);
                replace(&mut canvas_img, &temp_img, new_x as i64, new_y as i64);
            }
        }
    }
    canvas_img.save("io/output/zoom/00002.jpeg").unwrap();
}

