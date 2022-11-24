use crate::mosaic;
use crate::quadrants;
use crate::lil_videos;
use crate::instruct;
use crate::path;
use crate::path::{QUADRANT_A, QUADRANT_B};
use image::DynamicImage;
use std::fs;
use std::time::Instant;
use std::process::Command;
use std::collections::HashMap;


fn wipe_output_dirs() {
    fs::remove_dir_all(path::input_dir(&QUADRANT_A));
    fs::remove_dir_all(path::output_dir(&QUADRANT_B));
    fs::create_dir(path::input_dir(&QUADRANT_A));
    fs::create_dir(path::output_dir(&QUADRANT_B));
}

pub fn transpose_every_frame (ins: &Vec<instruct::FrameSequence>, one_way: bool) {
    let now = Instant::now();
    wipe_output_dirs();
    
    let mut last_handoff_info: &mut Option<mosaic::TransposeMakeMosaicReturn> = 
        &mut Option::None;
    
    let mut lil_imgs_map: HashMap<&String, Vec<mosaic::ImageInfo>> = HashMap::new();

    let mut total_frame_idx = 1;
    for sequence in ins {
        for seq_frame_idx in 1..sequence.total_frames + 1 {
            let frame_number_with_zeroes = path::prepend_zeroes(total_frame_idx);
            match &sequence.mode {
                instruct::SequenceMode::NoModification => {
                    // TODO this does not currently transpose, but only copies B frames
                    copy_original_img(&frame_number_with_zeroes, &path::QUADRANT_B);
                },
                instruct::SequenceMode::Mosaic(mosaic_instructions) => {
                    if let Some(lil_imgs_dir_str) =  &mosaic_instructions.lil_imgs_dir {
                        if !lil_imgs_map.contains_key(&lil_imgs_dir_str) {
                            let lil_imgs = mosaic::get_lil_imgs_from_dir(
                                    lil_imgs_dir_str, 5);
                            lil_imgs_map.insert(&lil_imgs_dir_str, lil_imgs);
                        }
                    }
                    let depth = mosaic_instructions.get_current_depth(
                        seq_frame_idx as u16, 
                        sequence.total_frames);
                    println!("depth: {}", depth);
                    let make_mosaic_return = 
                        transpose_one_mosaic_frame(
                            frame_number_with_zeroes,
                            depth,
                            mosaic_instructions.lil_imgs_dir.clone(),
                            lil_imgs_map.get(
                                &mosaic_instructions.lil_imgs_dir.as_ref().unwrap()),
                            one_way);
                    last_handoff_info.replace(make_mosaic_return.clone());
                    
                },
                instruct::SequenceMode::LittleVideos => {
                    match &last_handoff_info {
                        None => {
                            println!("no handoff_info received!");
                            // TODO this is where code might go to find the handoff frame from
                            // the future
                        },
                        Some(make_mosaic_return) => {
                            println!("received handoff_info!");
                            println!("last_handoff_info prev_parent_tiles len {}", 
                                    make_mosaic_return.prev_parent_tiles.len());
                            transpose_one_lil_videos_frame(
                                frame_number_with_zeroes, make_mosaic_return.clone());
                        }
                    };
                }
            };
            total_frame_idx += 1;
        }
    }

    let elapsed_time = now.elapsed();
    println!("transpose_every_frame() took {} seconds.", elapsed_time.subsec_millis());
}

fn copy_original_img(frame_number_str: &String, target_quadrant: &path::Quadrant) {
    Command::new("cp")
            .arg(path::input_path(target_quadrant, frame_number_str))
            .arg(path::output_path(target_quadrant, frame_number_str))
            .spawn()
            .expect("ls command failed to start");
}

fn transpose_one_lil_videos_frame(
        frame_number: String,
        handoff_info: mosaic::TransposeMakeMosaicReturn) {
    render_lil_videos_from_quadrant_b_frame(frame_number.clone(), handoff_info.clone());
    render_lil_videos_from_quadrant_a_frame(frame_number.clone(), handoff_info.clone());
}
fn render_lil_videos_from_quadrant_a_frame(
        frame_number: String,
        handoff_info: mosaic::TransposeMakeMosaicReturn) {
    lil_videos::compose_one_lil_video_frame(
        frame_number,
        handoff_info.prev_target_quadrant,
        handoff_info.prev_target_tiles);
}
fn render_lil_videos_from_quadrant_b_frame(
        frame_number: String, 
        handoff_info: mosaic::TransposeMakeMosaicReturn) {
    lil_videos::compose_one_lil_video_frame(
        frame_number,
        handoff_info.prev_parent_quadrant,
        handoff_info.prev_parent_tiles);
}

fn transpose_one_mosaic_frame (
        frame_number: String,
        depth: u32,
        lil_imgs_dir: Option<String>,
        lil_imgs: Option<&Vec<mosaic::ImageInfo>>,
        one_way: bool) -> mosaic::TransposeMakeMosaicReturn {
    println!("lil_imgs_dir: {:?}", lil_imgs_dir);
    //TODO drill lil_imgs dir to make_mosaic() from here
    let make_mosaic_return = render_mosaic_from_quadrant_b_frame(
        frame_number.clone(),
        depth,
        lil_imgs_dir.clone());
    if !one_way {
        match lil_imgs_dir.clone() {
            None => {
                render_mosaic_from_quadrant_a_frame(
                    frame_number.clone(), Some(make_mosaic_return), depth, Option::None)
            },
            // TODO passing option::none here makes me have to reload emoji dir every render
            // This is v slow...
            Some(lil_imgs_dir_str) => {
                render_mosaic_from_quadrant_a_frame(
                    frame_number.clone(), Option::None, depth, lil_imgs_dir.clone())
            }
        }
    }
    // This never actually gets used, but is needed so that the fn signature is satisfied
    else {
        make_mosaic_return
    }
}

fn render_mosaic_from_quadrant_a_frame (
        frame_number: String,
        prev_return: Option<mosaic::TransposeMakeMosaicReturn>,
        depth: u32,
        lil_imgs_dir: Option<String>) -> mosaic::TransposeMakeMosaicReturn {
    render_still_mosaic_from_quadrant_frame(
        "a", frame_number, prev_return, depth, lil_imgs_dir)
}
fn render_mosaic_from_quadrant_b_frame (
        frame_number: String,
        depth: u32,
        lil_imgs_dir: Option<String>) -> mosaic::TransposeMakeMosaicReturn {
    render_still_mosaic_from_quadrant_frame(
        "b", frame_number, Option::None, depth, lil_imgs_dir)
}

fn render_still_mosaic_from_quadrant_frame(
        target_quadrant_dir: &str,
        frame_number: String,
        make_mosaic_return: Option<mosaic::TransposeMakeMosaicReturn>,
        depth: u32,
        lil_imgs_dir: Option<String>) -> mosaic::TransposeMakeMosaicReturn {
    let mut parent_quadrant_dir = String::new();
    if target_quadrant_dir == "a" {
        parent_quadrant_dir = String::from("b");
    } else {
        parent_quadrant_dir = String::from("a");
    }

    let ext: &str = ".jpeg";
    let ip_file_name = [frame_number.clone(), ext.to_string()].concat();
        
    let target_img_name = [
        String::from("input"),
        target_quadrant_dir.to_string(),
        ip_file_name.clone()
    ].join("/");
    let parent_img_name = [
        String::from("input"),
        parent_quadrant_dir.clone(),
        ip_file_name.clone()
    ].join("/");

//  populate_lil_imgs_dir(
//      img_from_path(parent_img_name),
//      parent_quadrant_dir.clone(),
//      target_quadrant_dir.to_string(),
//      frame_number.clone());
//  render_and_save_mosaic(
//      img_from_path(target_img_name), 
//      parent_quadrant_dir.clone(),
//      target_quadrant_dir.to_string(),
//      frame_number);
    compose_mosaic_from_paths(
        img_from_path(target_img_name), 
        false, 
        parent_quadrant_dir.to_string(),
        target_quadrant_dir.to_string(),
        frame_number,
        make_mosaic_return,
        depth,
        lil_imgs_dir)

}


//  fn populate_lil_imgs_dir(
//      parent_img: DynamicImage,
//      parent_quadrant_dir: String,
//      target_quadrant_dir: String,
//      frame_number: String) {

 //     fs::remove_dir_all([
 //         String::from("io/lil_imgs"), 
 //         parent_quadrant_dir.clone()
 //     ].join("/"));
 //     fs::create_dir([
 //         String::from("io/lil_imgs"), 
 //         parent_quadrant_dir.clone()
 //     ].join("/"));

//      compose_mosaic_from_paths(
//          parent_img, 
//          true, 
//          parent_quadrant_dir.clone(),
//          target_quadrant_dir.clone(),
//          frame_number)
//  }

//  fn render_and_save_mosaic(
//      target_img: DynamicImage,
//      parent_quadrant_dir: String,
//      target_quadrant_dir: String,
//      frame_number: String) {

 //     compose_mosaic_from_paths(
 //         target_img,
 //         false,
 //         target_quadrant_dir.clone(),
 //         target_quadrant_dir.clone(),
 //         frame_number)
 // }

fn img_from_path(path: String) -> DynamicImage {
    let io_dir_name = String::from("io");
    let full_path = [io_dir_name, path].join("/");
    mosaic::open_image(full_path)
}

fn compose_mosaic_from_paths(
        img: DynamicImage,
        only_make_lil_imgs: bool,
        parent_quadrant_dir: String,
        target_quadrant_dir: String,
        frame_number: String,
        previous_return: Option<mosaic::TransposeMakeMosaicReturn>,
        depth: u32,
        lil_imgs_dir: Option<String>) -> mosaic::TransposeMakeMosaicReturn {

    let (xt, yt) = (1920, 1080);
    let crop_details = mosaic::CropDetails {
        depth,
        x_buf: (xt % (xt / depth)) / 2,
        y_buf: (yt % (yt / depth)) / 2,
        total_y_imgs: yt / depth,
        total_x_imgs: xt / depth
    };

//    if only_make_lil_imgs == true {
        
//      mosaic::save_lil_img_dir(mosaic::OrigTileGenArgs {
//          img, 
//          c: crop_details,
//          save_images: true,
//          quadrant_dir: target_quadrant_dir
//      });
//        return;
//    }
//    else {
//    let save_imgs = false;
//
//    TODO lil_imgs_dir should only be passed conditionally if we want to open imgs from a dir
//  let lil_imgs_dir = [
//      String::from("io/lil_imgs"),
//      parent_quadrant_dir.clone()
//   ].join("/");

    //let lil_imgs_dir = Option::None;
    mosaic::make_mosaic(
        img,
        lil_imgs_dir, 
        crop_details,
        parent_quadrant_dir,
        target_quadrant_dir,
        frame_number,
        previous_return)
//    }
}

//rm -rf io/input/b && mkdir io/input/b && ffmpeg -ss 510 -t 1 -i "io/input/vid/c.mp4" -r 30.0 "io/input/b/%4d.jpeg"
//ffmpeg -r 30 -i io/output/a/%04d.jpeg -vb 20000k -c:v libx264 -pix_fmt yuv420p io/output/vid/a.mp4
