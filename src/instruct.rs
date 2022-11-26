use crate::lil_videos;
use crate::mosaic;

#[derive(Clone)]
pub struct MosaicInstructions {
    pub starting_depth: u32,
    pub ending_depth: u32,
    pub lil_imgs_dir: Option<String>
}

impl MosaicInstructions {
    pub fn get_current_depth (&self, seq_frame_idx: u16, total_seq_frames: u32) -> u32 {
        // This is a fraction percentage of the progress that's been made in the frame seq
        println!("starting get_current_depth()");
        let depth_step_size =
            (self.ending_depth as i32 - self.starting_depth as i32) as f32 / 
            total_seq_frames as f32;
        if depth_step_size == 0.0 {
            return self.starting_depth;
        }
        let current_depth = 
            (seq_frame_idx as f32 * depth_step_size) + self.starting_depth as f32;
        println!("current_depth: {}, depth_step_size: {}", current_depth, depth_step_size);
        current_depth as u32
    }
}

#[derive(Clone)]
pub enum SequenceMode {
    Mosaic(MosaicInstructions), // String option is for optional lil_imgs_dir 
    LittleVideos,
    NoModification
}

#[derive(Clone)]
pub struct FrameSequence {
    pub total_frames: u32,
    pub mode: SequenceMode,
}

impl FrameSequence {
    pub fn new(total_frames: u32, mode: SequenceMode) -> Self {
        Self { total_frames, mode }
    }
}


fn x_second_transition(x: u32) -> Vec<FrameSequence> {
    let mut two_sec_trans: Vec<FrameSequence> = Vec::new();
    two_sec_trans.push(FrameSequence::new(2, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 120,
        lil_imgs_dir: Option::Some("io/lil_imgs/sdg_more_colors_jpeg_small".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new(13, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 10,
        lil_imgs_dir: Option::Some("io/lil_imgs/sdg_more_colors_jpeg_small".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new((x-1) * 30, SequenceMode::NoModification));
    two_sec_trans.push(FrameSequence::new(13, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 10,
        ending_depth: 120,
        lil_imgs_dir: Option::Some("io/lil_imgs/sdg_more_colors_jpeg_small".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new(2, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 120,
        lil_imgs_dir: Option::Some("io/lil_imgs/sdg_more_colors_jpeg_small".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans
}

fn ten_two_transition() -> Vec<FrameSequence> {
    let mut ten_two: Vec<FrameSequence> = Vec::new();
    let two_sec_trans = x_second_transition(2);
    let ten_sec_trans = x_second_transition(10);
    ten_two.append(&mut ten_sec_trans.clone());
    for i in 0..5 {
        ten_two.append(&mut two_sec_trans.clone());
    }
    ten_two
}

fn bench_instructions() -> Vec<FrameSequence> {
    let mut bench: Vec<FrameSequence> = Vec::new();
    bench.push(FrameSequence::new(20, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 20,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    bench.push(FrameSequence::new(130, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 20,
        ending_depth: 2,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    bench.push(FrameSequence::new(5, SequenceMode::NoModification));
    bench.push(FrameSequence::new(145, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 20,
        ending_depth: 20,
        lil_imgs_dir: None
    })));
    bench.push(FrameSequence::new(50, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 20,
        ending_depth: 200,
        lil_imgs_dir: None
    })));
    bench.push(FrameSequence{
        total_frames: 200,
        mode: SequenceMode::LittleVideos
    });
    bench.push(FrameSequence::new(40, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 200,
        ending_depth: 10,
        lil_imgs_dir: None
    })));
    bench.push(FrameSequence::new(40, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 200,
        ending_depth: 40,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    bench.push(FrameSequence::new(20, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 40,
        ending_depth: 2,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    bench
}

pub fn quad_test() -> Vec<FrameSequence> {
    let mut quad: Vec<FrameSequence> = Vec::new();
    quad.push(FrameSequence::new(15, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 20,
        ending_depth: 20,
        lil_imgs_dir: Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    quad
}

pub fn get_instructions () -> Vec<FrameSequence> {
    let mut instructions: Vec<FrameSequence> = Vec::new();
//  let mut two_sec_trans = two_second_transition();
//  for i in 0..18 {
//      instructions.append(&mut two_sec_trans.clone());
//  }
    let mut ten_two_trans = ten_two_transition();
    for i in 0..18 {
       instructions.append(&mut ten_two_trans.clone());
    }

//  let mut bench = bench_instructions();
//  for i in 0..1 {
//     instructions.append(&mut bench.clone());
//  }
    instructions
}
