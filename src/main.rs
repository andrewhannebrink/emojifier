mod mosaic;
use image::DynamicImage;
use std::fs;

fn main() {
    transpose_every_frame()
}

fn transpose_every_frame () {
    let mut total_frames = 0;
    let total_a_frames = fs::read_dir("io/input/a").unwrap().count();
    let total_b_frames = fs::read_dir("io/input/b").unwrap().count();

    if total_a_frames < total_b_frames {
        total_frames = total_a_frames;
    } else {
        total_frames = total_b_frames;
    }

    for i in 0..total_frames {
        let frame_number: &str = &(i.to_string());
        transpose_one_frame(frame_number);
    }
}

fn transpose_one_frame (frame_number: &str) {
    render_from_quadrant_b_frame(frame_number);
    render_from_quadrant_a_frame(frame_number);
}

fn render_from_quadrant_a_frame (frame_number: &str) {
    render_still_from_quadrant_frame("a", frame_number);
}
fn render_from_quadrant_b_frame (frame_number: &str) {
    render_still_from_quadrant_frame("b", frame_number);
}

fn render_still_from_quadrant_frame(target_quadrant_dir: &str, frame_number: &str) {
    let mut parent_quadrant_dir = String::new();
    if target_quadrant_dir == "a" {
        parent_quadrant_dir = String::from("b");
    } else {
        parent_quadrant_dir = String::from("a");
    }

    let ip_file_name = [frame_number, ".jpeg"].concat();
        
    let target_img_name = [
        String::from("input"),
        target_quadrant_dir.to_string(),
        ip_file_name.clone()
    ].join("/");
    let parent_img_name = [
        String::from("input"),
        parent_quadrant_dir.clone(),
        ip_file_name.clone()
    ].join("/");

    populate_lil_imgs_dir(
        img_from_path(parent_img_name),
        parent_quadrant_dir.clone(),
        target_quadrant_dir.to_string(),
        frame_number);
    render_and_save_mosaic(
        img_from_path(target_img_name), 
        parent_quadrant_dir.clone(),
        target_quadrant_dir.to_string(),
        frame_number);

}

fn populate_lil_imgs_dir(
    parent_img: DynamicImage,
    parent_quadrant_dir: String,
    target_quadrant_dir: String,
    frame_number: &str) {

    fs::remove_dir_all([
        String::from("io/lil_imgs"), 
        parent_quadrant_dir.clone()
    ].join("/"));
    fs::create_dir([
        String::from("io/lil_imgs"), 
        parent_quadrant_dir.clone()
    ].join("/"));

    compose_mosaic_from_paths(
        parent_img, 
        true, 
        parent_quadrant_dir.clone(),
        target_quadrant_dir.clone(),
        frame_number)
}

fn render_and_save_mosaic(
    target_img: DynamicImage,
    parent_quadrant_dir: String,
    target_quadrant_dir: String,
    frame_number: &str) {

    compose_mosaic_from_paths(
        target_img,
        false,
        target_quadrant_dir.clone(),
        target_quadrant_dir.clone(),
        frame_number)
}

fn img_from_path(path: String) -> DynamicImage {
    let io_dir_name = String::from("io");
    let full_path = [io_dir_name, path].join("/");
    mosaic::open_image(full_path)
}

fn compose_mosaic_from_paths(
        img: DynamicImage,
        only_make_lil_imgs: bool,
        parent_quadrant_dir: String,
        target_quadrant_dir: String,
        frame_number: &str) {

    let depth = 32;
    let (xt, yt) = (1920, 1080);
    let crop_details = mosaic::CropDetails {
        depth: depth,
        x_buf: (xt % (xt / depth)) / 2,
        y_buf: (yt % (yt / depth)) / 2,
        total_y_imgs: yt / depth,
        total_x_imgs: xt / depth
    };

    if only_make_lil_imgs == true {
        
        mosaic::save_lil_img_dir(mosaic::OrigTileGenArgs {
            img, 
            c: crop_details,
            save_images: true,
            quadrant_dir: target_quadrant_dir
        });
        return;
    }
    else {
        let lil_imgs_dir = [
            String::from("io/lil_imgs"),
            parent_quadrant_dir.clone()
        ].join("/");
        let save_imgs = false;
        mosaic::make_mosaic(
            img,
            lil_imgs_dir,
            crop_details,
            parent_quadrant_dir,
            target_quadrant_dir,
            frame_number);
    }
}
