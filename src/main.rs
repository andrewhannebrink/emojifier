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

struct Color(u8, u8, u8);

struct ImageInfo {
    //img: DynamicImage,
    avg_color: Color
}

fn main() {
    let img_name = String::from("test3.jpeg");
    let img = open_image(img_name);
    let lil_imgs_dir = String::from("./op");
    let lil_imgs = get_lil_imgs(lil_imgs_dir, 1 as u8);
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


    let orig_tile_iter = orig_tile_gen(OrigTileGenArgs {
        img:resized_img,
        c: crop_details,
        save_images
    });

    //TODO newTileGen
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
//  def getlittleimgs(directory, skip = 5):
//      littleimgs = []
//      littleimgnames = os.listdir(directory)
//      for imgname in littleimgnames:
//          imgfile = image.open(directory + imgname).convert('rgb')
//          try:
//              imginfo = imageinfo(imgname, imgfile, skip)
//              littleimgs.append(imginfo)
//          except:
//              print 'skipping image: ' + imgname
//      return littleimgs

fn get_lil_imgs(lil_imgs_dir: String, skip: u8) -> Vec<ImageInfo> {
    let mut lil_imgs: Vec<ImageInfo> = Vec::new();
    let lil_img_names = fs::read_dir(lil_imgs_dir).unwrap();
    for name in lil_img_names {
        let img_path = name.unwrap().path().display().to_string();
        let img = open_image(img_path);
        lil_imgs.push(ImageInfo {
            avg_color: get_avg_rgb(&img, skip as u8)
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

// TODO
//  fn get_avg_color(img: &DynamicImage, color: u32, skip: u32) -> i32 {
//      let (w, h) = img.dimensions();
//      let pixels = img.pixels();
//      let mut i = 0;
//      let (mut x, mut y) = (0, 0);
//      let mut intensity_list: Vec<ImageInfo> = Vec::new();
//          
//      for pixel in pixels {
//          if color == 0 {
//          for x in 0..(xi / skip) {
//              for y in 0..(yi / skip) {
//                  let (r,g,b) = imgPixels[x*skip, y*skip]
//                  intensity_list.push(img_pixels)   
//                  let pixel_color = pixel.0;
//              }
//          }
//          }
//          if color == 1 {
//              let pixel_color = pixel.1;
//          }
//          if color == 2 {
//              let pixel_color = pixel.2;
//          }
//          println!("{:?}", pixel);
//          i = i + 1;
//      }


//      //println!("{} {}", x, y);
//      0
//  }

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
