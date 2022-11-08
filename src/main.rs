use image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;
use std::fs;

struct CropDetails {
    depth: u32,
    total_x_imgs: u32,
    total_y_imgs: u32,
    x_buf: u32,
    y_buf: u32
}

struct Color(i32, i32, i32);

struct ImageInfo {
    //img: DynamicImage,
    avg_color: Color
}

fn main() {
    let img_name = String::from("test3.jpeg");
    let img = open_image(img_name);
    make_mosaic(img, 13, true);
}

fn make_mosaic(
    img: DynamicImage, 
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


    orig_tile_gen(OrigTileGenArgs {
        img:resized_img,
        c: crop_details,
        save_images
    });
}

struct OrigTileGenArgs {
    img: DynamicImage,
    c: CropDetails,
    save_images: bool
}

fn orig_tile_gen(args: OrigTileGenArgs) -> std::vec::IntoIter<ImageInfo> {

    let skip = 5;
    let mut orig_tiles: Vec<ImageInfo> = Vec::new();
    // TODO this wont go here

    let mut i = 0;
    println!("{:?}", args.img.dimensions());
    for x in 0..args.c.total_x_imgs {
        for y in 0..args.c.total_y_imgs {
            let temp_img = args.img.crop_imm(
                x*args.c.depth + args.c.x_buf,
                y*args.c.depth + args.c.y_buf,
                args.c.depth, 
                args.c.depth);
                //(x+1)*c.depth - 1 + c.x_buf,
                //(y+1)*c.depth - 1 + c.y_buf);
            orig_tiles.push(ImageInfo {
                //img: temp_img,
                avg_color: get_avg_rgb(&temp_img, skip)
            });
            // TODO this will get taken out
                        //println!("{}", op_path);
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
            i = i + 1;
        }
    }
    orig_tiles.into_iter()
}

fn get_avg_rgb(img: &DynamicImage, skip: u32) -> Color {
    let r = get_avg_color(img, 0, skip);
    let g = get_avg_color(img, 1, skip);
    let b = get_avg_color(img, 2, skip);
    Color(r, g, b)
}

// TODO
fn get_avg_color(img: &DynamicImage, color: u32, skip: u32) -> i32 {
    let (x, y) = img.dimensions();
    //println!("{} {}", x, y);
    0
}

fn open_image(img_name: String) -> DynamicImage {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(img_name).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    //img.save("test2.png").unwrap();
    img
}
