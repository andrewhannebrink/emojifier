mod mosaic;

fn mosaicFromAToBComposedOfC() {

    //let only_make_lil_imgs = true;
    //let img_name = String::from("test5.jpeg");
    let img_name = String::from("test4.jpeg");
    let only_make_lil_imgs = false;
    let img = mosaic::open_image(img_name);

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
            save_images: true
        });
    }
    else {
        let save_imgs = false;
        mosaic::make_mosaic(img, lil_imgs_dir, crop_details);
    }
}
