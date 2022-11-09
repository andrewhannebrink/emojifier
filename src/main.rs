mod mosaic;
use image::DynamicImage;

fn main() {
    let target_img_name = String::from("input/a/0.jpeg");
    let parent_img_name = String::from("input/b/0.jpeg");

    populate_lil_imgs_dir(img_from_path(parent_img_name));
    render_and_save_mosaic(img_from_path(target_img_name));
}

fn populate_lil_imgs_dir(parent_img: DynamicImage) {
    compose_mosaic_from_paths(
        parent_img, 
        true, 
        String::from("b"));
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

    let lil_imgs_dir = String::from("./op");

    let depth = 16;
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
        let save_imgs = false;
        mosaic::make_mosaic(img, lil_imgs_dir, crop_details, quadrant_dir);
    }
}
