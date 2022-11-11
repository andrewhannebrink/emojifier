use image::DynamicImage;
use image::imageops::FilterType;
use image::imageops::resize;
use image::imageops::replace;

pub fn frames_into_quadrants() {
    let mut view_port_img = image::open("io/input/a/0001.jpeg").unwrap();
    compose_quadrants_frame(view_port_img);
}

fn compose_quadrants_frame(mut view_port_img: DynamicImage) {
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
//  let frame_with_quadrant_a_cropped = view_port_img.crop_imm(
//      0,
//      0,
//      1920 / 2,
//      1080 / 2);
    view_port_img.save("io/output/quadrants/0001.jpeg").unwrap();
}
