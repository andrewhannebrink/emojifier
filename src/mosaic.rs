use image::GenericImageView;
use image::DynamicImage;
use crate::zoom;
use image::imageops::FilterType;
use image::imageops::replace;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct CropDetails {
    pub depth: u32,
    pub total_x_imgs: u32,
    pub total_y_imgs: u32,
    pub x_buf: u32,
    pub y_buf: u32
}

#[derive(Clone, Debug)]
struct Color(u8, u8, u8);

// TODO perhaps use lifetime params instead of clone()
#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub img: DynamicImage,
    avg_color: Color,
    pub parent_coords: (u32, u32),
    pub target_coords: Vec<(i64, i64)>,
}

fn parent_img_path (parent_quadrant_dir: String, frame_number: String) -> String {
    [
        ["io/input".to_string(), parent_quadrant_dir, frame_number].join("/"),
        "jpeg".to_string()
    ].join(".")
}

pub enum Quadrant {
    A, B, C, D
}

#[derive(Clone)]
pub struct TransposeMakeMosaicReturn {
    pub prev_parent_quadrant: String,
    pub prev_target_quadrant: String,
    pub prev_parent_tiles: Vec<ImageInfo>,
    pub prev_target_tiles: Vec<ImageInfo>,
    pub depth: u32
}

pub fn make_mosaic(
    img: DynamicImage,
    existing_lil_imgs: Option<&Vec<ImageInfo>>,
    crop_details: CropDetails,
    parent_quadrant_dir: String,
    target_quadrant_dir: String,
    frame_number: String,
    output_frame_number: String,
    previous_return: Option<TransposeMakeMosaicReturn>) -> TransposeMakeMosaicReturn {

    let now = Instant::now();

    let (lil_imgs, orig_tiles_iter) = match previous_return.clone() {
        None => {
            (
                match existing_lil_imgs {
                    None => {
                        get_lil_imgs_from_img(
                            parent_img_path(parent_quadrant_dir.clone(), frame_number.clone()),
                            crop_details.clone())
                    },
                    Some(previous_lil_imgs) => {
                        //println!("==------------getting lil_imgs from dir !");
                        println!("==------------using existing lil_imgs!");
                        previous_lil_imgs.to_vec()
                        //get_lil_imgs_from_dir(
                        //    &lil_imgs_dir_str, 5)
                    }
                },
                orig_tile_gen(OrigTileGenArgs {
                    img: img.clone(), // TODO make this a mutable reference
                    c: crop_details.clone(),
                    save_images: false,
                    quadrant_dir: target_quadrant_dir.clone()
                })
            )
        },
        Some(make_mosaic_return) => {
            (
                make_mosaic_return.prev_target_tiles,
                make_mosaic_return.prev_parent_tiles.into_iter()
            )
        } 
    };

    //todo figure out how to reuse crop_details from above using lifetime params
    let (new_tiles, resized_lil_imgs) = new_tiles_gen(NewTileGenArgs {
        c: crop_details.clone(),
        orig_tiles: orig_tiles_iter.clone(),
        lil_imgs: lil_imgs.clone(),
    });
    //todo figure out how to reuse crop_details from above using lifetime params
    let op_file_name = [frame_number.clone(), ".jpeg".to_string()].concat();
    let updated_mosaic_return = write_final_img(WriteFinalImageArgs {
        c: crop_details.clone(),
        new_tiles,
        orig_tiles: orig_tiles_iter.collect(),
        resized_lil_imgs,
        lil_imgs: lil_imgs.clone(),
        dest_path: [
            String::from("io/output"),
            target_quadrant_dir.clone(),
            op_file_name
        ].join("/"),
        target_quadrant_dir: target_quadrant_dir.clone(),
        parent_quadrant_dir: parent_quadrant_dir.clone(),
        frame_number,
        canvas_img: img
    });

    let elapsed_time = now.elapsed();
    println!("make_mosaic() took {} seconds.", elapsed_time.subsec_millis());

    // debug loop TODO remove
//  for prev_parent_tile in updated_mosaic_return.clone().prev_parent_tiles {
//      match prev_parent_tile.target_coords {
//          None => {
//              continue
//          },
//          Some(target_coords) => {
//              println!("parent coords in mm are: {:?}", prev_parent_tile.parent_coords);
//              println!("target coords in mm are: {:?}", prev_parent_tile.target_coords);
//          }
//      }
//  }

    updated_mosaic_return
}

struct WriteFinalImageArgs {
    c: CropDetails,
    new_tiles: std::vec::IntoIter<u32>,
    lil_imgs: Vec<ImageInfo>,
    resized_lil_imgs: Vec<ImageInfo>,
    orig_tiles: Vec<ImageInfo>, // Really just here to pass back to return statement
    dest_path: String,
    target_quadrant_dir: String,
    parent_quadrant_dir: String,
    frame_number: String,
    canvas_img: DynamicImage,
}
fn write_final_img(mut args: WriteFinalImageArgs) -> TransposeMakeMosaicReturn {
    let now = Instant::now();
    let mut lil_img_zoom_info: Vec<zoom::ZoomImageInfo> = vec![];

    let mut final_img: DynamicImage;
//  if !args.return_zoom_info {
//      // TODO this should be handled in path module
//      let final_img_file_name = [args.frame_number, ".jpeg".to_string()].concat();
//      let final_img_dir = [
//          "io/input".to_string(),
//          args.target_quadrant_dir.clone()
//      ].join("/");
//      final_img = open_image([
//          final_img_dir,
//          final_img_file_name
//      ].join("/"));
//  } else {
    final_img = args.canvas_img;
//    }

    //dbg!("{:?}", args.c.clone());

    for y in 0..args.c.total_y_imgs {
        for x in 0..args.c.total_x_imgs {
            let index_in_lil_imgs = args.new_tiles.next().unwrap();
            let target_coords = (
                (x*args.c.depth + args.c.x_buf) as i64, 
                (y*args.c.depth + args.c.y_buf) as i64
            );

            // resize the lil_img if the dimensions are not already correct. This is here
            // for when lil_imgs_dir is passed to make_mosaic, and the img size is not 
            // necessarily the depth, as would be the case when lil_imgs_dir is ot passed
            //
//          if args.lil_imgs[index_in_lil_imgs as usize].img.dimensions().0 != args.c.depth {
//              println!("resizing img");
//              resized_img = Some(args.lil_imgs[index_in_lil_imgs as usize].img.resize(
//                  args.c.depth, args.c.depth, FilterType::Gaussian));
//          }

            replace(&mut final_img, &args.resized_lil_imgs[index_in_lil_imgs as usize].img, 
                    target_coords.0, target_coords.1);

            // TODO update lil_imgs target_coords here
            args.lil_imgs[index_in_lil_imgs as usize].target_coords.push(target_coords);
            //dbg!("{:?}", args.lil_imgs[index_in_lil_imgs as usize].target_coords);
        }
    }
    //println!("dest_path: {}", args.dest_path.clone());
    final_img.save(args.dest_path.clone()).unwrap();

    println!("final image written to {}", args.dest_path);
    let elapsed_time = now.elapsed();
    println!("write_final_img() took {} seconds.", elapsed_time.subsec_millis());

    TransposeMakeMosaicReturn {
        prev_parent_quadrant: args.parent_quadrant_dir,
        prev_target_quadrant: args.target_quadrant_dir,
        prev_parent_tiles: args.lil_imgs.clone(),
        prev_target_tiles: args.orig_tiles,
        depth: args.c.depth
    }
}

struct NewTileGenArgs {
    orig_tiles: std::vec::IntoIter<ImageInfo>,
    c: CropDetails,
    lil_imgs: Vec<ImageInfo>
}

fn new_tiles_gen(mut args: NewTileGenArgs) -> (std::vec::IntoIter<u32>, Vec<ImageInfo>) {
    let now = Instant::now();
    let mut new_tiles: Vec<u32> = Vec::new();
    for orig_tile in args.orig_tiles {
        let new_tile = get_closest_img(&orig_tile, &mut (args.lil_imgs), &args.c);
        new_tiles.push(new_tile as u32);
    }
    let new_tiles_iter = new_tiles.into_iter();

    let elapsed_time = now.elapsed();
    println!("new_tiles_gen() took {} seconds.", elapsed_time.subsec_millis());

    (new_tiles_iter, args.lil_imgs)
}

fn get_closest_img(orig_tile: &ImageInfo, lil_imgs: &mut Vec<ImageInfo>, c: &CropDetails) -> usize {
    let mut closest_img_index = 0;

    let mut min_square_dis = 256 * 256 * 256; // TODO in V1 this was 3*266*266 which seems wrong
    for lil_img in lil_imgs.into_iter().enumerate() {
        let dis = 
            (
                (lil_img.1.avg_color.0 as i32 - orig_tile.avg_color.0 as i32) *
                (lil_img.1.avg_color.0 as i32 - orig_tile.avg_color.0 as i32)
            ) +
            (
                (lil_img.1.avg_color.1 as i32 - orig_tile.avg_color.1 as i32) *
                (lil_img.1.avg_color.1 as i32 - orig_tile.avg_color.1 as i32)
            ) +
            (
                (lil_img.1.avg_color.2 as i32 - orig_tile.avg_color.2 as i32) *
                (lil_img.1.avg_color.2 as i32 - orig_tile.avg_color.2 as i32)
            );
        if dis <= min_square_dis {
            min_square_dis = dis;
            closest_img_index = lil_img.0;
        }
        // Try to optimize by approximating
//      if min_square_dis < 1000 {
//          break
//      }
    }

    let closest_img = &lil_imgs[closest_img_index];
    let lil_img_size = closest_img.img.dimensions().0;
    if c.depth != lil_img_size {
        //TODO count how many imgs were resized and print
        //println!("lil_img is wrong size!");
        lil_imgs[closest_img_index].img = lil_imgs[closest_img_index].img.resize(
            c.depth, c.depth, FilterType::Gaussian);
    }

    closest_img_index
}


pub struct OrigTileGenArgs {
    pub img: DynamicImage,
    pub c: CropDetails,
    pub save_images: bool,
    pub quadrant_dir: String
}
fn orig_tile_gen(args: OrigTileGenArgs) -> std::vec::IntoIter<ImageInfo> {
    let now = Instant::now();

    let skip = 5;
    let mut orig_tiles: Vec<ImageInfo> = Vec::new();

    let mut i = 0;
    for y in 0..args.c.total_y_imgs {
        for x in 0..args.c.total_x_imgs {
            let parent_coords = (
                x*args.c.depth + args.c.x_buf,
                y*args.c.depth + args.c.y_buf
            );
            let temp_img = args.img.crop_imm(
                parent_coords.0,
                parent_coords.1,
                args.c.depth, 
                args.c.depth);
            if args.save_images {
                let op_dir = [
                        String::from("io/lil_imgs"),
                        args.quadrant_dir.clone()
                ].join("/");
                let op_ext = String::from("jpeg");
                let op_file = [i.to_string(), op_ext].join(".");
                let op_path = [op_dir, op_file].join("/");
                temp_img.save(op_path).unwrap();
            }
            if !args.save_images {
                //println!("parent_coords: {:?}", parent_coords);
                orig_tiles.push(ImageInfo {
                    avg_color: get_avg_rgb(&temp_img, skip),
                    img: temp_img,
                    parent_coords,
                    target_coords: Vec::new()
                });
            }
            i = i + 1;
        }
    }
    let elapsed_time = now.elapsed();
    println!("orig_tile_gen() took {} seconds.", elapsed_time.subsec_millis());

    orig_tiles.into_iter()
}

fn get_lil_imgs_from_img(parent_img_path: String, c: CropDetails) -> Vec<ImageInfo> {
    let now = Instant::now();

    let parent_img = open_image(parent_img_path);
    
    //TODO rename orig_tile_gen to just tile_gen?
    let lil_imgs = orig_tile_gen(OrigTileGenArgs {
        img: parent_img,
        c,
        save_images: false,
        quadrant_dir: "".to_string()});
    //TODO the above save_images + quadrant_dir should be an enum
    let elapsed_time = now.elapsed();
    println!("get_lil_imgs_from_img() took {} seconds.", elapsed_time.subsec_millis());

    lil_imgs.collect()
}

pub fn get_lil_imgs_from_dir(
        lil_imgs_dir: &String,
        skip: u8) -> Vec<ImageInfo> {
    let now = Instant::now();

    let mut lil_imgs: Vec<ImageInfo> = Vec::new();
    let lil_img_names = fs::read_dir(lil_imgs_dir).unwrap();
    for name in lil_img_names {
        let img_path = name.unwrap().path().display().to_string();
        let img = image::open(img_path).unwrap();

        lil_imgs.push(ImageInfo {
            avg_color: get_avg_rgb(&img, skip as u8),
            img,
            parent_coords: (0, 0),
            target_coords: Vec::new() // TODO parent_coords are not relevant for getting from dir
        });
    }
    let elapsed_time = now.elapsed();
    println!("get_lil_imgs() took {} seconds.", elapsed_time.subsec_millis());

    lil_imgs
}

fn get_avg_rgb(img: &DynamicImage, skip: u8) -> Color {
    let pixels = img.pixels();
    let mut i = 0;
    let mut red_sum: u32 = 0;
    let mut green_sum: u32 = 0;
    let mut blue_sum: u32 = 0;

    for pixel in pixels {
        red_sum = red_sum + pixel.2.0[0] as u32;
        green_sum = green_sum + pixel.2.0[1] as u32;
        blue_sum = blue_sum + pixel.2.0[2] as u32;  
        i = i + 1;
    }
    let red_avg = (red_sum / i) as u8;
    let green_avg = (green_sum / i) as u8;
    let blue_avg = (blue_sum / i) as u8;
    Color(red_avg, green_avg, blue_avg)
}

pub fn open_image(img_name: String) -> DynamicImage {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    println!("{}", img_name);
    let img = image::open(img_name).unwrap();
    img
}
