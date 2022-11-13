pub struct DepthChange {
    pub starting_depth: u8,
    pub ending_depth: u8,
}

pub enum SequenceMode {
    Mosaic(DepthChange),
    LittleVideos
}

pub struct FrameSequence {
    pub total_frames: u16,
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
