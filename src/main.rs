mod transpose;
mod mosaic;
mod lil_videos;
mod zoom_instruct;
mod scroll;
mod instruct;
mod path;
mod quadrants;
mod zoom;
mod ffmpeg_cmds;
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
    one_way: bool,

    #[arg(short, long)]
    quadrants: bool,

    #[arg(short, long)]
    benchmark_ten_sec: bool
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("minutes: {:?}", cli.minutes);
    println!("zoom_only: {:?}", cli.zoom_only);
    println!("transpose_only: {:?}", cli.transpose_only);
    println!("one_way: {:?}", cli.one_way);
    println!("quadrants: {:?}", cli.quadrants);
    println!("benchmark_ten_sec: {:?}", cli.benchmark_ten_sec);

    let now = Instant::now();

    if !cli.transpose_only {
        zoom::make_zooms("io/lil_imgs/emoji_big_buffered", cli.minutes);
    }
    if !cli.zoom_only {
        transpose_then_make_quadrants(cli.one_way, cli.minutes, cli.quadrants, 
            cli.benchmark_ten_sec).await;
    }
    //TODO put this at the end of transpose_every_frame()
    ffmpeg_cmds::make_video_from_dir(&path::QUADRANT_B, &"auto-test.mp4".to_string()).await;
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}

async fn transpose_then_make_quadrants(one_way: bool, minutes: u8, compose_quadrants: bool, benchmark: bool) {
    let instructions;
    if !benchmark {
        instructions = instruct::get_instructions(minutes);
    } else {
        instructions = instruct::benchmark();
    }
    transpose::transpose_every_frame(&instructions, one_way).await;
    if !one_way && compose_quadrants {
        quadrants::frames_into_quadrants();
    }
}

