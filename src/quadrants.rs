use image::DynamicImage;
use image::imageops::FilterType;
use image::imageops::resize;
use image::imageops::replace;

pub fn frames_into_quadrants() {
    for i in 0..5 { 
        compose_one_quadrants_frame(i);
    }
}

fn compose_one_quadrants_frame(frame_int: i32) {
    let frame_str = prepend_zeroes(frame_int as usize);
    let mut view_port_img = image::open("io/input/a/0001.jpeg").unwrap();
    let mut quadrant_a_img = image::open("io/input/a/0001.jpeg").unwrap();
    let mut quadrant_b_img = image::open("io/input/b/0001.jpeg").unwrap();
    let mut quadrant_c_img = image::open("io/output/a/0001.jpeg").unwrap();
    let mut quadrant_d_img = image::open("io/output/b/0001.jpeg").unwrap();
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
    view_port_img.save("io/output/quadrants/0001.jpeg").unwrap();
}

// TODO this is copy pasted from mosaic.rs. FIX THAT
fn prepend_zeroes(i: usize) -> String {
    let frame_number_without_zeroes: &str = &i.to_string();
    let mut zeroes_to_prepend = "000";
    if i >= 10 {
        zeroes_to_prepend = "00";
    } else if i >= 100 {
        zeroes_to_prepend = "0";
    } else if i >= 1000 {
        zeroes_to_prepend = "";
    }
    [zeroes_to_prepend, frame_number_without_zeroes].concat()
}
