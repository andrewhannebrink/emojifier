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
  zoom_coords: (f32, f32),
  depth: f32,
  out_of_view: bool
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
                zoom_coords: (x as f32 * sx, y as f32 * sy),
                depth: sx,
                out_of_view: false
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
    let mut canvas_img: RgbaImage = plain_white_img();
    let mut zoom_imgs = all_lil_imgs_img(lil_imgs_dir);
    for i in 2..32 {
        zoom_one_frame(i, &mut zoom_imgs, &mut canvas_img.clone());
    }
}

fn zoom_one_frame(
        frame_int: i32, zoom_imgs: &mut Vec<ZoomImageInfo>, canvas_img: &mut RgbaImage) {
    let z = 1.06;
    let (b, d) = (960_f32, 540_f32);
    println!("zoom_imgs length: {}", zoom_imgs.len());
    let mut t = 0;
    for i in 0..zoom_imgs.len() {
        if zoom_imgs[i].out_of_view {
            //println!("img out of view");
            continue
        }
        //dbg!(zoom_imgs[i].zoom_coords);
        let x = zoom_imgs[i].zoom_coords.0 as f32;
        let y = zoom_imgs[i].zoom_coords.1 as f32;
        let new_x = z * x + (b - b*z);
        let new_y = z * y + (d - d*z);
        let new_x_int = new_x.round() as i32;
        let new_y_int = new_y.round() as i32;
        //dbg!(new_x, new_y);
        let prev_size = zoom_imgs[i].depth;
        let new_size = z * prev_size;
        let new_size_int = new_size.round() as u32;
        zoom_imgs[i].zoom_coords = (new_x, new_y);
        zoom_imgs[i].depth = new_size;

        // if x or y is out of bounds do nothing
        if new_y_int + new_size_int as i32 >= 0 && new_y_int <= DIMENSIONS.1 as i32 {
            if new_x_int + new_size_int as i32 >= 0 && new_x_int <= DIMENSIONS.0 as i32 {
                //println!("new coords: {}, {}", new_x_int, new_y_int);
                let temp_img = zoom_imgs[i].img.resize(
                    new_size_int, new_size_int, FilterType::Gaussian);
                replace(canvas_img, &temp_img, new_x as i64, new_y as i64);
                t = t + 1;
            } else { zoom_imgs[i].out_of_view = true; }
        } else { zoom_imgs[i].out_of_view = true; }
    }
    println!("cropped {} imgs", t);
    //TODO test for commit
    let frame_number_str = path::prepend_zeroes(frame_int);
    println!("{}", frame_number_str);
    canvas_img.save(path::zoom_output_path(&frame_number_str)).unwrap();
}

