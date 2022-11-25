mod transpose;
mod mosaic;
mod lil_videos;
mod quadrants;
mod instruct;
mod path;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let n = path::prepend_zeroes(1452);
    println!("{}", n);
    let instructions = instruct::get_instructions();
    transpose::transpose_every_frame(&instructions, true).await;
    //transpose::transpose_every_frame(&instructions, false);
    //quadrants::frames_into_quadrants();
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}
