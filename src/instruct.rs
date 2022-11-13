pub struct DepthChange {
    pub starting_depth: u32,
    pub ending_depth: u32,
}

impl DepthChange {
    pub fn get_current_depth (&self, seq_frame_idx: u16, total_seq_frames: u32) -> u32 {
        // This is a fraction percentage of the progress that's been made in the frame seq
        let depth_step_size =
            (self.ending_depth as i32 - self.starting_depth as i32) as f32 / 
            total_seq_frames as f32;
        if depth_step_size as i32 == 0 {
            return self.starting_depth;
        }
        let current_depth = 
            (seq_frame_idx as f32 * depth_step_size) + self.starting_depth as f32;
        println!("current_depth: {}, depth_step_size: {}", current_depth, depth_step_size);
        current_depth as u32
    }
}

pub enum SequenceMode {
    Mosaic(DepthChange),
    LittleVideos
}

pub struct FrameSequence {
    pub total_frames: u32,
    pub mode: SequenceMode,
}

pub fn get_instructions () -> Vec<FrameSequence> {
    let mut instructions: Vec<FrameSequence> = Vec::new();
    instructions.push(FrameSequence{
        total_frames: 5,
        mode: SequenceMode::Mosaic(DepthChange {
            starting_depth: 120,
            ending_depth: 60 
        })
    });
    instructions.push(FrameSequence{
        total_frames: 25,
        mode: SequenceMode::Mosaic(DepthChange {
            starting_depth: 60,
            ending_depth: 90 
        })
    });
    instructions
}
