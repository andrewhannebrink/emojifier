mod transpose;
mod mosaic;
mod lil_videos;
mod quadrants;
mod instruct;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let instructions = instruct::get_instructions();
    transpose::transpose_every_frame(&instructions, true);
    //transpose::transpose_every_frame(&instructions, false);
    //quadrants::frames_into_quadrants();
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}
