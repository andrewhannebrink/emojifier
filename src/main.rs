use image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;

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
    let img_name = String::from("brightcolors.jpeg");
    let img = open_image(img_name);
    make_mosaic(img, 13);
}

fn make_mosaic(
    img: DynamicImage, 
    depth: u32) {

    let (xt, yt) = (1920, 1080);
    let (xi, yi) = img.dimensions();
    let crop_details = CropDetails {
        depth: depth,
        x_buf: (xt % (xt / depth)) / 2,
        y_buf: (yt % (yt / depth)) / 2,
        total_y_imgs: yt / depth,
        total_x_imgs: xt / depth
    };

    let resized_img = img.resize(xt, yt, FilterType::Nearest);

    println!("{} {} {} {}", 
             crop_details.x_buf, 
             crop_details.y_buf, 
             crop_details.total_x_imgs,
             crop_details.total_y_imgs);

    orig_tile_gen(resized_img, crop_details);
}


fn orig_tile_gen(
    img: DynamicImage,
    c: CropDetails
    ) -> std::vec::IntoIter<ImageInfo> {

    let skip = 5;
    let mut orig_tiles: Vec<ImageInfo> = Vec::new();
    // TODO this wont go here
    let op_dir = String::from("op");

    let i = 0;
    for x in 0..c.total_x_imgs {
        for y in 0..c.total_y_imgs {
            let temp_img = img.crop_imm(
                x*c.depth + c.x_buf,
                y*c.depth + c.y_buf,
                c.depth, 
                c.depth);
                //(x+1)*c.depth - 1 + c.x_buf,
                //(y+1)*c.depth - 1 + c.y_buf);
            orig_tiles.push(ImageInfo {
                //img: temp_img,
                avg_color: get_avg_rgb(temp_img, skip)
            });
            // TODO this will get taken out
            let op_name = i.to_string();
            let op_path = [op_dir, op_name].join("/");
            temp_img.save(op_path).unwrap();
        }
    }
    orig_tiles.into_iter()
}

fn get_avg_rgb(img: DynamicImage, skip: u32) -> Color {
    let r = get_avg_color(&img, 0, skip);
    let g = get_avg_color(&img, 1, skip);
    let b = get_avg_color(&img, 2, skip);
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
