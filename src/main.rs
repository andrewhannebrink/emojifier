use image::{GenericImage, GenericImageView, Rgb, Rgba, RgbImage, RgbaImage, GrayImage};
use image::DynamicImage;
use image::imageops::FilterType;
use image::imageops::replace;
use std::fs;

struct CropDetails {
    depth: u32,
    total_x_imgs: u32,
    total_y_imgs: u32,
    x_buf: u32,
    y_buf: u32
}

#[derive(Clone)]
struct Color(u8, u8, u8);

// TODO perhaps use lifetime params instead of clone()
#[derive(Clone)]
struct ImageInfo {
    img: DynamicImage,
    avg_color: Color
}

fn main() {
    let img_name = String::from("test3.jpeg");
    let img = open_image(img_name);
    let lil_imgs_dir = String::from("./op");
    let lil_imgs = get_lil_imgs(lil_imgs_dir, 1 as u8);
    make_mosaic(img, lil_imgs, 25, true);
}

fn make_mosaic(
    img: DynamicImage, 
    lil_imgs: Vec<ImageInfo>,
    depth: u32,
    save_images: bool) {

    let (xt, yt) = (1920, 1080);
    let (xi, yi) = img.dimensions();
    let crop_details = CropDetails {
        depth: depth,
        x_buf: (xt % (xt / depth)) / 2,
        y_buf: (yt % (yt / depth)) / 2,
        total_y_imgs: yt / depth,
        total_x_imgs: xt / depth
    };

    println!("{} {}", xt, yt);
    let resized_img = img.resize(xt, yt, FilterType::Gaussian);

    println!("{} {} {} {}", 
             crop_details.x_buf, 
             crop_details.y_buf, 
             crop_details.total_x_imgs,
             crop_details.total_y_imgs);


    let orig_tiles_iter = orig_tile_gen(OrigTileGenArgs {
        img: resized_img,
        c: crop_details,
        save_images
    });
    //TODO figure out how to reuse crop_details from above using lifetime params
    let new_tiles = new_tiles_gen(NewTileGenArgs {
        c: CropDetails {
            depth: depth,
            x_buf: (xt % (xt / depth)) / 2,
            y_buf: (yt % (yt / depth)) / 2,
            total_y_imgs: yt / depth,
            total_x_imgs: xt / depth
        },
        orig_tiles: orig_tiles_iter,
        lil_imgs: lil_imgs.clone(),
    });
    //TODO figure out how to reuse crop_details from above using lifetime params
    write_final_img(WriteFinalImageArgs {
        c: CropDetails {
            depth: depth,
            x_buf: (xt % (xt / depth)) / 2,
            y_buf: (yt % (yt / depth)) / 2,
            total_y_imgs: yt / depth,
            total_x_imgs: xt / depth
        },
        new_tiles,
        lil_imgs: lil_imgs.clone()
    });
}

struct WriteFinalImageArgs {
    c: CropDetails,
    new_tiles: std::vec::IntoIter<usize>,
    lil_imgs: Vec<ImageInfo>
}
fn write_final_img(args: WriteFinalImageArgs) {
    //TODO write this method
    //TODO do not hardcode this
    let (w, h) = (1920, 1080);
    let buffer = RgbaImage::new(w, h);
    let final_img_view: &dyn GenericImageView<Pixel=Rgba<u8>> = &buffer;
    let final_img = final_img_view.view(0, 0, 1920, 1080);

    let mut i = 0;
    for y in 0..args.c.total_y_imgs {
        for x in 0..args.c.total_x_imgs {
            let new_tile = &args.lil_imgs[i];
            replace(final_img, &args.lil_imgs[i].img, 
                    (x*args.c.depth + args.c.x_buf) as i64, 
                    (y*args.c.depth + args.c.y_buf) as i64);
            i += 1;
        }
    }
    final_img.save("op.png").unwrap();
}

struct NewTileGenArgs {
    orig_tiles: std::vec::IntoIter<ImageInfo>,
    c: CropDetails,
    lil_imgs: Vec<ImageInfo>
}

//TODO <String should be number>
fn new_tiles_gen(args: NewTileGenArgs) -> std::vec::IntoIter<usize> {
    let mut new_tiles: Vec<usize> = Vec::new();
    for orig_tile in args.orig_tiles {
        let new_tile = get_closest_img(&orig_tile, &(args.lil_imgs));
        new_tiles.push(new_tile);
    }
    new_tiles.into_iter()
}

fn get_closest_img(orig_tile: &ImageInfo, lil_imgs: &Vec<ImageInfo>) -> usize {
    println!("{:?}", orig_tile.img);
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
            println!("closest_img_index? {}, distance: {}", lil_img.0, dis);
            closest_img_index = lil_img.0;
            if dis == 0 {
                println!("DISTANCE of 0... lil_img_colors: {:?}, orig_tile_colors: {:?}", 
                        (lil_img.1.avg_color.0, lil_img.1.avg_color.1, lil_img.1.avg_color.2),
                        (orig_tile.avg_color.0, orig_tile.avg_color.1, orig_tile.avg_color.2));
            }

        }
    }
    let closest_img = &lil_imgs[closest_img_index];
    //println!("closest_img.avg_color: {:?}, distance {}", 
    //            (closest_img.avg_color.0, closest_img.avg_color.1, closest_img.avg_color.2),
    //            min_square_dis);
    closest_img_index
}

struct OrigTileGenArgs {
    img: DynamicImage,
    c: CropDetails,
    save_images: bool
}
fn orig_tile_gen(args: OrigTileGenArgs) -> std::vec::IntoIter<ImageInfo> {

    let skip = 5;
    let mut orig_tiles: Vec<ImageInfo> = Vec::new();

    let mut i = 0;
    println!("{:?}", args.img.dimensions());
    for y in 0..args.c.total_y_imgs {
        for x in 0..args.c.total_x_imgs {
            let temp_img = args.img.crop_imm(
                x*args.c.depth + args.c.x_buf,
                y*args.c.depth + args.c.y_buf,
                args.c.depth, 
                args.c.depth);
                //(x+1)*c.depth - 1 + c.x_buf,
                //(y+1)*c.depth - 1 + c.y_buf);
            if args.save_images {
                fs::remove_dir("op");
                fs::create_dir("op");
                let op_dir = String::from("op");
                let op_ext = String::from("png");
                let op_name = i.to_string();
                let op_file = [op_name, op_ext].join(".");
                let op_path = [op_dir, op_file].join("/");
                temp_img.save(op_path).unwrap();
            }
            orig_tiles.push(ImageInfo {
                avg_color: get_avg_rgb(&temp_img, skip),
                img: temp_img
            });
            i = i + 1;
        }
    }
    orig_tiles.into_iter()
}

fn get_lil_imgs(lil_imgs_dir: String, skip: u8) -> Vec<ImageInfo> {
    let mut lil_imgs: Vec<ImageInfo> = Vec::new();
    let lil_img_names = fs::read_dir(lil_imgs_dir).unwrap();
    for name in lil_img_names {
        let img_path = name.unwrap().path().display().to_string();
        let img = open_image(img_path);
        lil_imgs.push(ImageInfo {
            avg_color: get_avg_rgb(&img, skip as u8),
            img: img
        });
    }
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
        //println!("{:?}", pixel.2.0);
        red_sum = red_sum + pixel.2.0[0] as u32;
        green_sum = green_sum + pixel.2.0[1] as u32;
        blue_sum = blue_sum + pixel.2.0[2] as u32;  
        //println!("{:?}", pixel);
        i = i + 1;
    }
    let red_avg = (red_sum / i) as u8;
    let green_avg = (green_sum / i) as u8;
    let blue_avg = (blue_sum / i) as u8;
    Color(red_avg, green_avg, blue_avg)
}

fn open_image(img_name: String) -> DynamicImage {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(img_name).unwrap();

    // The dimensions method returns the images width and height.
    //println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    //println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    //img.save("test2.png").unwrap();
    img
}
