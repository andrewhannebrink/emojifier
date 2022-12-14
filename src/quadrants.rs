use image::imageops::FilterType;
use image::imageops::resize;
use image::imageops::replace;
use std::fs;
use crate::path;
use std::time::Instant;

pub fn frames_into_quadrants() {
    fs::remove_dir_all("io/output/quadrants");
    fs::create_dir("io/output/quadrants");
    let total_frames = fs::read_dir("io/output/a").unwrap().count();
    for i in 1..total_frames + 1 { 
        compose_one_quadrants_frame(i as i32);
    }
}

fn compose_one_quadrants_frame(frame_int: i32) {
    let now = Instant::now();

    let frame_str = path::prepend_zeroes(frame_int);
    println!("{}", frame_str);
    let mut view_port_img = 
        image::open(file_path(frame_str.clone(), "io/input/a")).unwrap();
    let mut quadrant_a_img = image::open(file_path(
            frame_str.clone(),
            "io/input/a")).unwrap();
    let mut quadrant_b_img = image::open(file_path(
            frame_str.clone(),
            "io/input/b")).unwrap();
    let mut quadrant_c_img = image::open(file_path(
            frame_str.clone(),
            "io/output/a")).unwrap();
    let mut quadrant_d_img = image::open(file_path(
            frame_str.clone(),
            "io/output/b")).unwrap();
    let quadrant_a_img_resized = resize(&mut quadrant_a_img,
        1920/2, 1080/2, FilterType::Nearest);
    let quadrant_b_img_resized = resize( &mut quadrant_b_img,
        1920/2, 1080/2, FilterType::Nearest);
    let quadrant_c_img_resized = resize( &mut quadrant_c_img,
        1920/2, 1080/2, FilterType::Nearest);
    let quadrant_d_img_resized = resize( &mut quadrant_d_img,
        1920/2, 1080/2, FilterType::Nearest);
    replace(&mut view_port_img, &quadrant_a_img_resized, 0, 0);
    replace(&mut view_port_img, &quadrant_b_img_resized, 1920/2, 0);
    replace(&mut view_port_img, &quadrant_c_img_resized, 0, 1080/2);
    replace(&mut view_port_img, &quadrant_d_img_resized, 1920/2, 1080/2);
    view_port_img.save(file_path(frame_str, "io/output/quadrants")).unwrap();

    let elapsed_time = now.elapsed();
    println!("compose_one_quadrants_frame() took {} seconds.", elapsed_time.subsec_millis());
}

fn file_name(frame_str: String) -> String {
    [frame_str, ".jpeg".to_string()].concat()
}

fn file_path(frame_str: String, quadrant_src_dir: &str) -> String {
    let name = file_name(frame_str);
    [quadrant_src_dir.to_string(), name].join("/")
}
