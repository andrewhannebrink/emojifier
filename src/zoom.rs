use crate::mosaic;
use image::{ImageBuffer, RgbaImage, DynamicImage};
use image::imageops::FilterType;
use image::imageops::replace;
use crate::path;
use std::fs;
use std::collections::HashMap;
static DIMENSIONS: (u32, u32) = (1920, 1080);

#[derive(Debug, Clone)]
pub struct ZoomImageInfo {
    pub img: DynamicImage,
    pub zoom_coords: Vec<(f32, f32)>,
    pub depth: f32,
    pub out_of_view: bool
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

fn all_lil_imgs_img(lil_imgs_dir: &str) -> (Vec<ZoomImageInfo>, Vec<mosaic::ImageInfo>) {
    wipe_zoom_dir();
    let mut canvas_img: RgbaImage = plain_white_img();
    let lil_imgs = mosaic::get_lil_imgs_from_dir(&lil_imgs_dir.to_string(), 5);
    let mut zoom_imgs: Vec<ZoomImageInfo> = Vec::new();
    // set a central pixel to white
    
    // Makes a white image
    let n = lil_imgs.len() as f32;
    let x = DIMENSIONS.0 as f32;
    let y = DIMENSIONS.1 as f32;
    let px = (n * x / y).sqrt().ceil();
    let py = (n * y / x).sqrt().ceil();

    let sx: f32;
    let sy: f32;
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
//  for lil_img in lil_imgs {
//      for coords in lil_img.target_coords {
            if i >= lil_imgs.len() {
                break
            }
            zoom_imgs.push(ZoomImageInfo {
                img: lil_imgs[i].img.clone(), //TODO satisfying the borrow checker here is hard
                zoom_coords: vec![(x as f32 * sx, y as f32 * sy)],
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
    (zoom_imgs, lil_imgs)
}

pub fn zoom(lil_imgs_dir: &str) {
    let canvas_img: RgbaImage = plain_white_img();
    let (mut zoom_imgs, lil_imgs) = all_lil_imgs_img(lil_imgs_dir);
    // TODO this should probably go insid the for loop
    let mut zoom_return = 
            zoom_one_frame(2, &mut zoom_imgs, &mut canvas_img.clone());
    for i in 3..150 {
        if zoom_return.depth < 200 {
            zoom_return = zoom_one_frame(i, &mut zoom_imgs, &mut canvas_img.clone());
        } else {
            let mosaic_depth = 8;
            zoom_return.depth = mosaic_depth;
            zoom_imgs = mosaic::make_mosaic(
                zoom_return.output_img.clone(),
                Some(&lil_imgs),
                mosaic::CropDetails {
                    depth: mosaic_depth,
                    x_buf: (DIMENSIONS.0 % (DIMENSIONS.0 / mosaic_depth)) /2,
                    y_buf: (DIMENSIONS.1 % (DIMENSIONS.1 / mosaic_depth)) /2,
                    total_x_imgs: DIMENSIONS.0 / mosaic_depth,
                    total_y_imgs: DIMENSIONS.1 / mosaic_depth
                },
                "zoom".to_string(),
                "zoom".to_string(),
                path::prepend_zeroes(i),
                true,
                None).lil_img_zoom_info;
        }
    }
}

struct ZoomOneFrameReturn {
    output_img: DynamicImage,
    depth: u32
}
fn zoom_one_frame(
        frame_int: i32, zoom_imgs: &mut Vec<ZoomImageInfo>, canvas_img: &mut RgbaImage) 
        -> ZoomOneFrameReturn {
    let z = 1.05;
    //let (b, d) = (960_f32, 540_f32);
    let (b, d) = (640_f32, 360_f32);
    println!("zoom_imgs length: {}", zoom_imgs.len());
    let mut t = 0;
    let mut total_out_of_view = 0;
    let mut zoom_depth: u32 = 0;
    for mut zoom_img in zoom_imgs {
        for mut zoom_coords in &zoom_img.zoom_coords {   
            if zoom_img.out_of_view {
                //println!("img out of view");
                total_out_of_view = total_out_of_view + 1;
                continue
            }
            //dbg!(zoom_imgs[i].zoom_coords);
            let x = zoom_coords.0 as f32;
            let y = zoom_coords.1 as f32;
            let new_x = z * x + (b - b*z);
            let new_y = z * y + (d - d*z);
            let new_x_int = new_x.round() as i32;
            let new_y_int = new_y.round() as i32;
            //dbg!(new_x, new_y);
            let prev_size = zoom_img.depth;
            let new_size = z * prev_size;
            let new_size_int = new_size.round() as u32;
            zoom_coords = &(new_x, new_y);
            zoom_img.depth = new_size;

            // if x or y is out of bounds do nothing
            if new_y_int + new_size_int as i32 >= 0 && new_y_int <= DIMENSIONS.1 as i32 {
                if new_x_int + new_size_int as i32 >= 0 && new_x_int <= DIMENSIONS.0 as i32 {
                    //println!("new coords: {}, {}", new_x_int, new_y_int);
                    let temp_img = zoom_img.img.resize(
                        new_size_int, new_size_int, FilterType::Gaussian);
                    replace(canvas_img, &temp_img, new_x as i64, new_y as i64);
                    t = t + 1;
                    zoom_depth = new_size_int;
                } else { zoom_img.out_of_view = true; }
            } else { zoom_img.out_of_view = true; }
        }
    }
    println!("cropped {} imgs with depth = {}px", t, zoom_depth);
    println!("total_out_of_view imgs skipped: {}", total_out_of_view);
    //TODO test for commit
    let frame_number_str = path::prepend_zeroes(frame_int);
    println!("{}", frame_number_str);
    canvas_img.save(path::zoom_output_path(&frame_number_str)).unwrap();

    ZoomOneFrameReturn {
        output_img: DynamicImage::ImageRgba8(canvas_img.clone()), //TODO i do not like the clone here
        depth: zoom_depth
    }
}

