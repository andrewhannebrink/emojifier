use image::{GenericImage, GenericImageView, Rgb, Rgba, RgbImage, RgbaImage, GrayImage};
use image::DynamicImage;
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
    img: DynamicImage,
    avg_color: Color,
    parent_coords: (u32, u32),
    //target_coords: Option<(u16, u16)>,

}

pub fn save_lil_img_dir(args: OrigTileGenArgs) {
    let now = Instant::now();

    fs::remove_dir_all("io/lil_imgs/a");
    fs::remove_dir_all("io/lil_imgs/b");
    fs::create_dir("io/lil_imgs/a");
    fs::create_dir("io/lil_imgs/b");
    orig_tile_gen(OrigTileGenArgs {
        img: args.img,
        c: args.c,
        save_images: args.save_images,
        quadrant_dir: args.quadrant_dir,
    });
    let elapsed_time = now.elapsed();
    println!("save_lil_img_dir() took {} seconds.", elapsed_time.subsec_millis());
}

fn parent_img_path (parent_quadrant_dir: String, frame_number: String) -> String {
    [
        ["io/input".to_string(), parent_quadrant_dir, frame_number].join("/"),
        "jpeg".to_string()
    ].join(".")
}

enum Quadrant {
    A, B, C, D
}

#[derive(Clone)]
pub struct MakeMosaicReturn {
    prev_parent_quadrant: String,
    prev_target_quadrant: String,
    pub prev_parent_tiles: Vec<ImageInfo>,
    pub prev_target_tiles: Vec<ImageInfo>
}

pub fn make_mosaic(
    img: DynamicImage,
    lil_imgs_dir: String,
    crop_details: CropDetails,
    parent_quadrant_dir: String,
    target_quadrant_dir: String,
    frame_number: String,
    previous_return: Option<MakeMosaicReturn>) -> MakeMosaicReturn {

    let now = Instant::now();

    //let lil_imgs: Vec<ImageInfo> = get_lil_imgs_from_dir(lil_imgs_dir.clone(), 1 as u8);

    let (lil_imgs, orig_tiles_iter) = match previous_return.clone() {
        None => {
            (
                get_lil_imgs_from_img(
                    parent_img_path(parent_quadrant_dir.clone(), frame_number.clone()),
                    crop_details.clone()),
                orig_tile_gen(OrigTileGenArgs {
                    img,
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

    //TODO figure out how to reuse crop_details from above using lifetime params
    let mut new_tiles = new_tiles_gen(NewTileGenArgs {
        c: crop_details.clone(),
        orig_tiles: orig_tiles_iter.clone(),
        lil_imgs: lil_imgs.clone(),
    });
    //TODO figure out how to reuse crop_details from above using lifetime params
    let op_file_name = [frame_number.clone(), ".jpeg".to_string()].concat();
    write_final_img(WriteFinalImageArgs {
        c: crop_details.clone(),
        new_tiles,
        lil_imgs: lil_imgs.clone(),
        dest_path: [
            String::from("io/output"),
            target_quadrant_dir.clone(),
            op_file_name
        ].join("/"),
        target_quadrant_dir: target_quadrant_dir.clone(),
        frame_number
    });

    let elapsed_time = now.elapsed();
    println!("make_mosaic() took {} seconds.", elapsed_time.subsec_millis());

    MakeMosaicReturn {
        prev_parent_quadrant: parent_quadrant_dir,
        prev_target_quadrant: target_quadrant_dir,
        prev_parent_tiles: lil_imgs.clone(),
        prev_target_tiles: orig_tiles_iter.collect() //TODO fix this
    }
}

struct WriteFinalImageArgs {
    c: CropDetails,
    new_tiles: std::vec::IntoIter<u32>,
    lil_imgs: Vec<ImageInfo>,
    dest_path: String,
    target_quadrant_dir: String,
    frame_number: String

}
fn write_final_img(mut args: WriteFinalImageArgs) {
    let now = Instant::now();

    let final_img_file_name = [args.frame_number, ".jpeg".to_string()].concat();
    let final_img_dir = [
        "io/input".to_string(),
        args.target_quadrant_dir
    ].join("/");
    let mut final_img = open_image([
        final_img_dir,
        final_img_file_name
    ].join("/"));
    let (target_w, target_h) = final_img.dimensions();

    //dbg!("{:?}", args.c.clone());

    let mut i = 0;
    for y in 0..args.c.total_y_imgs {
        for x in 0..args.c.total_x_imgs {
            let index_in_lil_imgs = args.new_tiles.next().unwrap();
            replace(&mut final_img, &args.lil_imgs[index_in_lil_imgs as usize].img, 
                    (x*args.c.depth + args.c.x_buf) as i64, 
                    (y*args.c.depth + args.c.y_buf) as i64);
            i += 1;
        }
    }
    //println!("dest_path: {}", args.dest_path.clone());
    final_img.save(args.dest_path.clone()).unwrap();

    println!("final image written to {}", args.dest_path);
    let elapsed_time = now.elapsed();
    println!("write_final_img() took {} seconds.", elapsed_time.subsec_millis());
}

struct NewTileGenArgs {
    orig_tiles: std::vec::IntoIter<ImageInfo>,
    c: CropDetails,
    lil_imgs: Vec<ImageInfo>
}

fn new_tiles_gen(args: NewTileGenArgs) -> std::vec::IntoIter<u32> {
    let now = Instant::now();
    let mut new_tiles: Vec<u32> = Vec::new();
    for orig_tile in args.orig_tiles {
        let new_tile = get_closest_img(&orig_tile, &(args.lil_imgs));
        new_tiles.push(new_tile as u32);
    }
    let mut new_tiles_iter = new_tiles.into_iter();

    let elapsed_time = now.elapsed();
    println!("new_tiles_gen() took {} seconds.", elapsed_time.subsec_millis());

    new_tiles_iter
}

fn get_closest_img(orig_tile: &ImageInfo, lil_imgs: &Vec<ImageInfo>) -> usize {
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
                println!("parent_coords: {:?}", parent_coords);
                orig_tiles.push(ImageInfo {
                    avg_color: get_avg_rgb(&temp_img, skip),
                    img: temp_img,
                    parent_coords
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

fn get_lil_imgs_from_dir(lil_imgs_dir: String, skip: u8) -> Vec<ImageInfo> {
    let now = Instant::now();

    let mut lil_imgs: Vec<ImageInfo> = Vec::new();
    let lil_img_names = fs::read_dir(lil_imgs_dir).unwrap();
    for name in lil_img_names {
        let img_path = name.unwrap().path().display().to_string();
        let img = open_image(img_path);
        lil_imgs.push(ImageInfo {
            avg_color: get_avg_rgb(&img, skip as u8),
            img: img,
            parent_coords: (0, 0) // TODO parent_coords are not relevant for getting from dir
        });
    }
    let elapsed_time = now.elapsed();
    println!("get_lil_imgs() took {} seconds.", elapsed_time.subsec_millis());

    lil_imgs
}

fn get_avg_rgb(img: &DynamicImage, skip: u8) -> Color {
    let (w, h) = img.dimensions();
    let pixels = img.pixels();
    let mut i = 0;
    let (mut x, mut y) = (0, 0);
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
    let img = image::open(img_name).unwrap();
    img
}

pub fn prepend_zeroes(i: usize) -> String {
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
