mod transpose;
mod mosaic;
mod lil_videos;
mod instruct;
mod path;
mod quadrants;
mod zoom;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    zoom::all_lil_imgs_img("io/lil_imgs/emoji_big_buffered/");
    //transpose_then_make_quadrants(true).await;

    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}

async fn transpose_then_make_quadrants(one_way: bool) {
    let instructions = instruct::get_instructions();
    //transpose::transpose_every_frame(&instructions, true).await;
    transpose::transpose_every_frame(&instructions, one_way).await;
    if !one_way {
        quadrants::frames_into_quadrants();
    }
}
