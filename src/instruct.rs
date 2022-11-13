pub struct DepthChange {
    pub starting_depth: u32,
    pub ending_depth: u32,
}

impl DepthChange {
    pub fn get_current_depth (&self, seq_frame_idx: u16) -> u32 {
        // TODO increment according to seq_frame_idx instead of hardcoding
        self.starting_depth
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
        total_frames: 1,
        mode: SequenceMode::Mosaic(DepthChange {
            starting_depth: 90,
            ending_depth: 90
        })
    });
    instructions.push(FrameSequence{
        total_frames: 29,
        mode: SequenceMode::Mosaic(DepthChange {
            starting_depth: 120,
            ending_depth: 120
        })
    });
    instructions
}
