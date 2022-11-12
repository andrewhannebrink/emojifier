mod mosaic;
mod quadrants;
use image::DynamicImage;
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    wipe_output_dirs();
    transpose_every_frame();
    quadrants::frames_into_quadrants();
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}

fn wipe_output_dirs() {
    fs::remove_dir_all("io/output/a");
    fs::remove_dir_all("io/output/b");
    fs::create_dir("io/output/a");
    fs::create_dir("io/output/b");
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

    for i in 1..total_frames + 1 {
        let frame_number_with_zeroes = mosaic::prepend_zeroes(i);
        transpose_one_frame(frame_number_with_zeroes);
    }
}

fn transpose_one_frame (frame_number: String) {
    render_from_quadrant_b_frame(frame_number.clone());
    render_from_quadrant_a_frame(frame_number.clone());
}

fn render_from_quadrant_a_frame (frame_number: String) {
    render_still_from_quadrant_frame("a", frame_number);
}
fn render_from_quadrant_b_frame (frame_number: String) {
    render_still_from_quadrant_frame("b", frame_number);
}

fn render_still_from_quadrant_frame(target_quadrant_dir: &str, frame_number: String) {
    let mut parent_quadrant_dir = String::new();
    if target_quadrant_dir == "a" {
        parent_quadrant_dir = String::from("b");
    } else {
        parent_quadrant_dir = String::from("a");
    }

    let ext: &str = ".jpeg";
    let ip_file_name = [frame_number.clone(), ext.to_string()].concat();
        
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
        frame_number.clone());
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
    frame_number: String) {

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
    frame_number: String) {

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
        frame_number: String) {

    let depth = 60;
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

//rm -rf io/input/b && mkdir io/input/b && ffmpeg -ss 510 -t 1 -i "io/input/vid/c.mp4" -r 30.0 "io/input/b/%4d.jpeg"
//ffmpeg -r 30 -i io/output/a/%04d.jpeg -vb 20000k -c:v libx264 -pix_fmt yuv420p io/output/vid/a.mp4
