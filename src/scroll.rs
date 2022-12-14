use crate::zoom;
use crate::path;
use image::{RgbaImage, DynamicImage, GenericImageView};
use image::imageops::replace;
use image::imageops::FilterType;
use crate::path::{QUADRANT_A, QUADRANT_B};
use rand::seq::SliceRandom;
use rand::Rng;


pub fn scroll_one_frame(
        frame_int: i32,
        zoom_imgs: &mut Vec<zoom::ZoomImageInfo>,
        canvas_img: &mut RgbaImage,
        direction: (f32, f32),
        velocity: f32,
        quadrant: &path::Quadrant) -> zoom::ZoomOneFrameReturn {
    let mut zoom_depth = 0_u32;
    let mut refill_coords: Vec<(i32, i32)> = vec![];
    for mut zoom_img in zoom_imgs.iter_mut() {
        let mut zoom_coords_indices_to_remove: Vec<usize> = vec![];
        for (i, zoom_coords) in zoom_img.zoom_coords.iter_mut().enumerate() {
            let new_x = zoom_coords.0 + direction.0 * velocity;
            let new_y = zoom_coords.1 + direction.1 * velocity;
            let new_x_int = new_x.round() as i32;
            let new_y_int = new_y.round() as i32;
            *zoom_coords = (new_x, new_y);

            //TODO this loop should be its own fn. it also occurs in zoom_one_frame()
            if new_y_int + zoom_depth as i32 >= 0 && new_y_int <= zoom::DIMENSIONS.1 as i32 {
                if new_x_int + zoom_depth as i32 >= 0 && new_x_int <= zoom::DIMENSIONS.0 as i32 {
                    zoom_depth = zoom_img.depth as u32;
                    //println!("zoom_depth 1 = {}", zoom_depth);
                    replace(canvas_img, &zoom_img.resized_img, new_x as i64, new_y as i64);
                    refill_coords.append(&mut get_refill_coords(
                        (new_x_int, new_y_int),
                        zoom_depth, 
                        direction));
                } else { 
                    zoom_coords_indices_to_remove.push(i);
                }
            } else { 
                zoom_coords_indices_to_remove.push(i);
            }
        }
        for i in zoom_coords_indices_to_remove.iter().rev() {
            zoom_img.zoom_coords.remove(*i);
        }
    }
    for coords in refill_coords {
        //println!("COORDS TO REFILL = {}, {}", coords.0, coords.1);
        //println!("zoom_depth = {}", zoom_depth);
        //println!("zoom_imgs length = {}", &zoom_imgs.len());
        //TODO this should select an image from zoom_imgs randomly or methodically,
        //not hardcoded
        if !already_refilled(coords, &zoom_imgs) {
            let mut rng = rand::thread_rng();
            let random_img_idx = rng.gen_range(0..zoom_imgs.len());
            zoom_imgs[random_img_idx].zoom_coords.push((coords.0 as f32, coords.1 as f32));
            zoom_imgs[random_img_idx].depth = zoom_depth as f32;
            if zoom_imgs[random_img_idx].resized_img.dimensions().0 != zoom_depth {
                zoom_imgs[random_img_idx].resized_img = zoom_imgs[random_img_idx].img.resize(
                    zoom_depth, zoom_depth, FilterType::Gaussian);
            }
            replace(canvas_img, &zoom_imgs[random_img_idx].resized_img, coords.0 as i64, coords.1 as i64);
        }
    }
    let frame_number_str = path::prepend_zeroes(frame_int);
    canvas_img.save(path::zoom_output_path(&frame_number_str, quadrant)).unwrap();

    zoom::ZoomOneFrameReturn {
        output_img: DynamicImage::ImageRgba8(canvas_img.clone()),
        depth: zoom_depth
    }
}

fn already_refilled(zoom_coords: (i32, i32), zoom_imgs: &Vec<zoom::ZoomImageInfo>) -> bool {
    let mut already_refilled = false;
    for zoom_img in zoom_imgs.iter() {
        for coords in zoom_img.zoom_coords.iter() {
            if (coords.0 as i32 - zoom_coords.0).abs() < 2 && 
                    (coords.1 as i32 - zoom_coords.1).abs() < 2 {
                already_refilled = true;
            }
        }
    }
    already_refilled 
}

fn get_refill_coords<'a>(
        zoom_int_coords: (i32, i32),
        depth: u32,
        scroll_direction: (f32, f32)) -> Vec<(i32, i32)> {

    let mut coords_to_refill: Vec<(i32, i32)> = vec![];
    if scroll_direction.0 > 0.0 {
        if scroll_direction.1 > 0.0 {
            //println!("scroll coords: {}, {}", zoom_int_coords.0, zoom_int_coords.1);
            if zoom_int_coords.1 > 0 && zoom_int_coords.1 <= depth as i32 {
                //println!("new image in view at ({}, {})", zoom_int_coords.0, zoom_int_coords.1);
                //println!("depth is = {}", depth);
                coords_to_refill.push((
                    zoom_int_coords.0,
                    zoom_int_coords.1 - depth as i32));
                //replace(canvas_img, &zoom_imgs[60].img, 
                //    zoom_int_coords.0 as i64, (zoom_int_coords.1 + depth as i32) as i64);

            }
            if zoom_int_coords.0 > 0 && zoom_int_coords.0 <= depth as i32 {
                coords_to_refill.push((
                    zoom_int_coords.0 - depth as i32,
                    zoom_int_coords.1
                ))
            }

        } else if scroll_direction.1 <= 0.0 {

        }
    }
    if scroll_direction.0 <= 0.0 {

        if scroll_direction.1 > 0.0 {

        } else if scroll_direction.1 <= 0.0 {

        }
    }
    coords_to_refill
}
