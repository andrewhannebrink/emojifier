mod mosaic;
use image::DynamicImage;
use std::fs;

fn main() {
    render_from_quadrant_b_frame();
    //render_from_quadrant_a_frame();
}

fn render_from_quadrant_a_frame () {
    render_still_from_quadrant_frame(String::from("a"));
}
fn render_from_quadrant_b_frame () {
    render_still_from_quadrant_frame(String::from("b"));
}

fn render_still_from_quadrant_frame(target_quadrant_dir: String) {
    let mut parent_quadrant_dir = String::new();
    if target_quadrant_dir == "a" {
        parent_quadrant_dir = String::from("b");
    } else {
        parent_quadrant_dir = String::from("a");
    }
        
    let target_img_name = [
        String::from("input"),
        target_quadrant_dir.clone(),
        String::from("0.jpeg")
    ].join("/");
    let parent_img_name = [
        String::from("input"),
        parent_quadrant_dir.clone(),
        String::from("0.jpeg")
    ].join("/");

    populate_lil_imgs_dir(
        img_from_path(parent_img_name),
        parent_quadrant_dir.clone());
    render_and_save_mosaic(img_from_path(target_img_name));

}

fn populate_lil_imgs_dir(parent_img: DynamicImage, parent_quadrant_dir: String) {

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
        parent_quadrant_dir.clone());
}

fn render_and_save_mosaic(target_img: DynamicImage) {
    compose_mosaic_from_paths(
        target_img,
        false,
        String::new());
}

fn img_from_path(path: String) -> DynamicImage {
    let io_dir_name = String::from("io");
    let full_path = [io_dir_name, path].join("/");
    mosaic::open_image(full_path)
}

fn compose_mosaic_from_paths(
        img: DynamicImage,
        only_make_lil_imgs: bool,
        quadrant_dir: String) {

    let mut parent_quadrant_dir = String::new();
    if quadrant_dir == "a" {
        parent_quadrant_dir = String::from("b");
    } else {
        parent_quadrant_dir = String::from("a");
    }

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
            quadrant_dir
        });
        return;
    }
    else {
        let lil_imgs_dir = [
            String::from("io/lil_imgs"),
            parent_quadrant_dir.clone()
        ].join("/");
        let save_imgs = false;
        mosaic::make_mosaic(img, lil_imgs_dir, crop_details, quadrant_dir);
    }
}
