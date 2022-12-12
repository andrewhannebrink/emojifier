use crate::mosaic;
use image::imageops::replace;
use image::DynamicImage;
use image::GenericImageView;

pub fn compose_one_lil_video_frame (
        //prev_parent_quadrant: mosaic::Quadrant,
        frame_number: String,
        prev_parent_quadrant: String,
        prev_parent_tiles: Vec<mosaic::ImageInfo>) {

    // TODO this is repeated code - make it one fn
    let mut target_quadrant_dir = String::new();
    if prev_parent_quadrant == "a" {
        target_quadrant_dir = String::from("b");
    } else {
        target_quadrant_dir = String::from("a");
    }
    let final_img_file_name = [frame_number, ".jpeg".to_string()].concat();
    //println!("frame_number: {}", frame_number)
    let img_to_repace_tiles_onto_path = [
            "io/input".to_string(),
            target_quadrant_dir.clone(),
            final_img_file_name.clone()
        ].join("/");
    println!("img_to_repace_tiles_onto_path: {}", img_to_repace_tiles_onto_path);
    let mut img_to_replace_tiles_onto = image::open(img_to_repace_tiles_onto_path).unwrap();

    let new_parent_img_path = [
        "io/input".to_string(),
        prev_parent_quadrant.clone(),
        final_img_file_name.clone()
    ].join("/");
    let new_parent_img = image::open(new_parent_img_path).unwrap();

    println!("length of prev_parent_tiles: {}", prev_parent_tiles.len());
    for prev_parent_tile in prev_parent_tiles {
        
        // Only load the next parent_img if it has target_coords
        let mut new_parent_tile: Option<DynamicImage> = Option::None;
        if prev_parent_tile.target_coords.len() > 0 {
            let depth = prev_parent_tile.img.dimensions().0;
            let new_lil_img = new_parent_img.crop_imm(
                prev_parent_tile.parent_coords.0,
                prev_parent_tile.parent_coords.1,
                depth, depth);
            new_parent_tile = Some(new_lil_img);
        }
        for target_coord in prev_parent_tile.target_coords {
            let new_cropped_tile = match &new_parent_tile {
                Some(cropped_img) => {
                    cropped_img
                }, 
                None => {
                    println!("BIG PROBLEM! we should have received an image here.");
                    &prev_parent_tile.img
                }
            };
            replace(&mut img_to_replace_tiles_onto, new_cropped_tile,
                    target_coord.0, target_coord.1);
        }
    }
    let dest_path = [
        "io/output".to_string(),
        target_quadrant_dir,
        final_img_file_name
    ].join("/");

    img_to_replace_tiles_onto.save(dest_path.clone());
    println!("lil vid frame saved at {}", dest_path);
}
