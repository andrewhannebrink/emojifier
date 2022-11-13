use crate::mosaic;

pub fn compose_one_lil_video_frame (
        //prev_parent_quadrant: mosaic::Quadrant,
        prev_parent_quadrant: String,
        frame_number: String,
        prev_parent_tiles: Vec<mosaic::ImageInfo>) {
    for prev_parent_tile in prev_parent_tiles {
        //dbg!("parent_coords in transpose.rs: {:?}", prev_parent_tile.parent_coords);
//      match prev_parent_tile.target_coords {
//          None => {
//              println!("NO TARGET_COORDS! parent_coords received in transpose.rs: {:?}", 
//                       prev_parent_tile.parent_coords);
//              //TODO
//          },
//          Some(target_coords) => {
//              println!("WE GOT TARGET COORDS MUFUCKA");
//              println!("target_coords received in transpose.rs: {:?}", target_coords);
//              println!("parent_coords received in transpose.rs: {:?}", 
//                       prev_parent_tile.parent_coords);
//          }
//      }
        println!("parent_coords received in transpose.rs: {:?}",
                     prev_parent_tile.parent_coords);
        for target_coord in prev_parent_tile.target_coords {
            println!("WE GOT TARGET COORDS MUFUCKA");
            println!("parent_quadrant_dir: {}", prev_parent_quadrant);
            println!("target_coord received in transpose.rs: {:?}", target_coord);
        }
    }
}
