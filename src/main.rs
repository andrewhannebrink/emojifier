mod transpose;
mod mosaic;
mod lil_videos;
mod zoom_instruct;
mod scroll;
mod instruct;
mod path;
mod quadrants;
mod zoom;
use std::time::Instant;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Emojifier")]
#[command(author = "Andrew Hannebrink")]
#[command(version = "1.0.0")]
#[command(about = "Rust Photo Mosaic Video Generator", long_about = None)]
struct Cli {
    #[arg(short, long)]
    minutes: u8,

    #[arg(short, long)]
    zoom_only: bool,

    #[arg(short, long)]
    transpose_only: bool,

    #[arg(short, long)]
    one_way: bool
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("minutes: {:?}", cli.minutes);
    println!("zoom_only: {:?}", cli.zoom_only);
    println!("transpose_only: {:?}", cli.transpose_only);
    println!("one_way: {:?}", cli.one_way);
    let now = Instant::now();
//  let _matches = clap::command!()
//      .version("1.0.0")
//      .author("Andrew Hannebrink")
//      .about("Photo Mosaic Video Generator")
//      .arg(
//          clap::arg!("zoom_only")
//              .short('z')
//              .help("Zoom/Scroll only")
//              .required(false)
//      )
//      .get_matches();


    zoom::make_zooms("io/lil_imgs/emoji_big_buffered");
    transpose_then_make_quadrants(false).await;
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}

async fn transpose_then_make_quadrants(one_way: bool) {
    let instructions = instruct::get_instructions();
    transpose::transpose_every_frame(&instructions, one_way).await;
    if !one_way {
        quadrants::frames_into_quadrants();
    }
}

