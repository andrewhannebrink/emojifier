use crate::zoom;
use crate::path;
use image::{RgbaImage, DynamicImage};
use crate::path::{QUADRANT_A, QUADRANT_B};


pub fn scroll_one_frame(
        fraome_int: i32,
        zoom_imgs: &mut Vec<zoom::ZoomImageInfo>,
        canvas_img: &mut RgbaImage,
        direction: (f32, f32),
        velocity: f32,
        quadrant: &path::Quadrant) -> zoom::ZoomOneFrameReturn {
    let mut zoom_depth = 0_u32;
    for mut zoom_img in zoom_imgs {
        zoom_depth = zoom_img.depth as u32;
        let mut zoom_coords_indices_to_remove: Vec<usize> = vec![];
        for (i, mut zoom_coords) in zoom_img.zoom_coords.iter_mut().enumerate() {
            let new_x = zoom_coords.0 + direction.0 * velocity;
            let new_y = zoom_coords.1 + direction.1 * velocity;
            let new_x_int = new_x.round() as i32;
            let new_y_int = new_y.round() as i32;
            *zoom_coords = (new_x, new_y);
            println!("scroll coords: {}, {}", zoom_coords.0, zoom_coords.1);
        }
    }
    zoom::ZoomOneFrameReturn {
        output_img: DynamicImage::ImageRgba8(canvas_img.clone()),
        depth: zoom_depth
    }
}

