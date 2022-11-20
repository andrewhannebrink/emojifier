use crate::lil_videos;
use crate::mosaic;

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

pub enum SequenceMode {
    Mosaic(MosaicInstructions), // String option is for optional lil_imgs_dir 
    LittleVideos,
    NoModification
}

pub struct FrameSequence {
    pub total_frames: u32,
    pub mode: SequenceMode,
}

impl FrameSequence {
    pub fn new(total_frames: u32, mode: SequenceMode) -> Self {
        Self { total_frames, mode }
    }
}

pub fn get_instructions () -> Vec<FrameSequence> {
    let mut instructions: Vec<FrameSequence> = Vec::new();
    instructions.push(FrameSequence::new(3, SequenceMode::NoModification));
    instructions.push(FrameSequence::new(3, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 30,
        ending_depth: 30,
        lil_imgs_dir: Option::Some("io/lil_imgs/sdg_jpeg_small".to_string())
    })));
    instructions
//  instructions.push(FrameSequence{
//      total_frames: 3,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 30,
//          ending_depth: 30,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence {
//      total_frames: 10,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 30,
//          ending_depth: 60,
//          lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 10,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 60,
//          ending_depth: 20,
//          lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 10,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 20,
//          ending_depth: 20,
//          lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 20,
//          ending_depth: 20,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 20,
//          ending_depth: 40,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 60,
//      mode: SequenceMode::LittleVideos
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 40,
//          ending_depth: 90,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 60,
//      mode: SequenceMode::LittleVideos
//  });
//  instructions.push(FrameSequence{
//      total_frames: 60,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 90,
//          ending_depth: 20,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::LittleVideos
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 20,
//          ending_depth: 20,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 20,
//          ending_depth: 180,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 180,
//          ending_depth: 180,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::Mosaic(MosaicInstructions {
//          starting_depth: 180,
//          ending_depth: 20,
//          lil_imgs_dir: Option::None
//      })
//  });
//  instructions.push(FrameSequence{
//      total_frames: 30,
//      mode: SequenceMode::LittleVideos
//  });
}
