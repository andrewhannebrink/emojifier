static GCF_DEPTHS: &'static [u32] = &[12, 24, 30, 40, 60, 120];
use rand::prelude::SliceRandom;

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

pub fn total_frames(sequences: &Vec<FrameSequence>) -> u32 {
    let mut total_frames = 0;
    for _instruction_set in sequences.iter() {
        total_frames = total_frames + _instruction_set.total_frames;
    }
    total_frames
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
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new(13, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 10,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new((x-1) * 30, SequenceMode::NoModification));
    two_sec_trans.push(FrameSequence::new(13, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 10,
        ending_depth: 120,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans.push(FrameSequence::new(2, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: 120,
        ending_depth: 120,
        lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
        //lil_imgs_dir: Option::Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    two_sec_trans
}

fn ten_two_transition() -> Vec<FrameSequence> {
    let mut ten_two: Vec<FrameSequence> = Vec::new();
    let two_sec_trans = x_second_transition(2);
    let ten_sec_trans = x_second_transition(10);
    ten_two.append(&mut ten_sec_trans.clone());
    for _i in 0..5 {
        ten_two.append(&mut two_sec_trans.clone());
    }
    ten_two
}

fn flat_emoji(depth: u32, seconds: u32) -> Vec<FrameSequence> {
    let mut flat: Vec<FrameSequence> = Vec::new();
    flat.push(FrameSequence::new(seconds * 30, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: depth,
        ending_depth: depth,
        lil_imgs_dir: Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    println!("flat_emoji length = {}", total_frames(&flat));
    flat
}
fn flat_splice(depth: u32, seconds: u32) -> Vec<FrameSequence> {
    let mut flat: Vec<FrameSequence> = Vec::new();
    flat.push(FrameSequence::new(seconds * 30, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth: depth,
        ending_depth: depth,
        lil_imgs_dir: None
    })));
    flat
}
fn lil_vid(seconds: u32) -> Vec<FrameSequence> {
    let mut lil_vid: Vec<FrameSequence> = Vec::new();
    lil_vid.push(FrameSequence{
        total_frames: seconds*30,
        mode: SequenceMode::LittleVideos
    });
    lil_vid
}
fn bump_emoji(starting_depth: u32, ending_depth: u32) -> Vec<FrameSequence> {
    let mut bump: Vec<FrameSequence> = Vec::new();
    bump.push(FrameSequence::new(30, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth,
        ending_depth,
        lil_imgs_dir: Some("io/lil_imgs/emoji_buffered".to_string())
    })));
    bump
}
fn bump_splice(starting_depth: u32, ending_depth: u32) -> Vec<FrameSequence> {
    let mut bump: Vec<FrameSequence> = Vec::new();
    bump.push(FrameSequence::new(30, SequenceMode::Mosaic(MosaicInstructions {
        starting_depth,
        ending_depth,
        lil_imgs_dir: None
    })));
    bump
}
fn spike_emoji(starting_depth: u32, spike_depth: u32) -> Vec<FrameSequence> {
    let mut spike = Vec::new();
    spike.append(&mut bump_emoji(starting_depth, spike_depth));
    spike.append(&mut bump_emoji(spike_depth, starting_depth));
    spike
}
fn spike_splice(starting_depth: u32, spike_depth: u32) -> Vec<FrameSequence> {
    let mut spike = Vec::new();
    spike.append(&mut bump_splice(starting_depth, spike_depth));
    spike.append(&mut bump_splice(spike_depth, starting_depth));
    spike
}
fn no_mod(seconds: u32) -> Vec<FrameSequence> {
    let mut no_mod = Vec::new();
    no_mod.push(FrameSequence::new(30 * seconds, SequenceMode::NoModification));
    no_mod
}

fn emoji_wobble() -> Vec<FrameSequence> {
    let mut bench: Vec<FrameSequence> = Vec::new();
    bench.append(&mut flat_emoji(30, 2));
    bench.append(&mut bump_emoji(30, 4));
    bench.append(&mut no_mod(9));

    println!("emoji_wobble length = {}", total_frames(&bench));
    bench
}
fn splice_wobble() -> Vec<FrameSequence> {
    let mut wobble: Vec<FrameSequence> = Vec::new();
    wobble.append(&mut flat_splice(30, 2));
    wobble.append(&mut bump_splice(30, 120));
    wobble.append(&mut flat_splice(120, 2));
    wobble.append(&mut bump_splice(120, 30));
    wobble.append(&mut flat_splice(30, 2));
    wobble.append(&mut bump_splice(60, 16));
    wobble.append(&mut flat_splice(16, 2));
    wobble.append(&mut bump_splice(16, 120));
    println!("splice_wobble length = {}", total_frames(&wobble));

    wobble
}
fn lil_vid_wobble(lil_video_depth: u32) -> Vec<FrameSequence> {
    let mut wobble: Vec<FrameSequence> = Vec::new();
    wobble.append(&mut flat_splice(30, 2));
    wobble.append(&mut bump_splice(30, lil_video_depth));
    wobble.append(&mut flat_splice(lil_video_depth, 2));
    wobble.append(&mut lil_vid(3));
    wobble.append(&mut flat_splice(lil_video_depth, 3));
    wobble.append(&mut bump_splice(lil_video_depth, 30));
    println!("lil_vid_wobble length = {}", total_frames(&wobble));
    wobble
}
fn splice_wave() -> Vec<FrameSequence> {
    let mut wave: Vec<FrameSequence> = Vec::new();
    wave.append(&mut bump_splice(30, 16));
    wave.append(&mut bump_splice(16, 120));
    wave.append(&mut bump_splice(120, 30));
    wave.append(&mut flat_splice(30, 9));
    println!("splice_wave length = {}", total_frames(&wave));
    wave
}


fn concise_bench() -> Vec<FrameSequence> {
    let mut concise_bench: Vec<FrameSequence> = Vec::new();
    concise_bench.append(&mut emoji_wobble());
    concise_bench.append(&mut lil_vid_wobble(120));
    concise_bench.append(&mut flat_emoji(30, 12));
    concise_bench.append(&mut splice_wave());
    concise_bench.append(&mut lil_vid_wobble(30));
    concise_bench
}
fn concise_bench_2(size_opt_1: Option<u32>, size_opt_2: Option<u32>, size_opt_3: Option<u32>) -> Vec<FrameSequence> {
    let mut concise_bench_2: Vec<FrameSequence> = Vec::new();
    let size_1 = match size_opt_1 {
        Some(dest_sz) => { dest_sz },
        None => { *GCF_DEPTHS.choose(&mut rand::thread_rng()).unwrap() }
    };
    let size_2 = match size_opt_2 {
        Some(dest_sz) => { dest_sz },
        None => { *GCF_DEPTHS.choose(&mut rand::thread_rng()).unwrap() }
    };
    let size_3 = match size_opt_3 {
        Some(dest_sz) => { dest_sz },
        None => { *GCF_DEPTHS.choose(&mut rand::thread_rng()).unwrap() }
    };
    concise_bench_2.append(&mut bump_splice(30, size_1));
    concise_bench_2.append(&mut flat_splice(size_1, 11));

    concise_bench_2.append(&mut lil_vid_wobble(size_1));

    concise_bench_2.append(&mut bump_splice(size_1, size_2));
    concise_bench_2.append(&mut flat_splice(size_2, 4));
    concise_bench_2.append(&mut bump_splice(size_2, 4));

    concise_bench_2.append(&mut no_mod(2));
    concise_bench_2.append(&mut bump_splice(4, size_2));
    concise_bench_2.append(&mut flat_splice(size_2, 3));

    concise_bench_2.append(&mut lil_vid_wobble(size_2));

    concise_bench_2.append(&mut bump_emoji(size_2, size_3));
    concise_bench_2.append(&mut flat_emoji(size_3, 10));
    concise_bench_2.append(&mut bump_emoji(size_3, 30));
    concise_bench_2
}

fn lil_video_micro_test() -> Vec<FrameSequence> {
    let mut micro: Vec<FrameSequence> = Vec::new();
    micro.append(&mut flat_splice(40, 1));
    micro.append(&mut lil_vid(1));
    micro.append(&mut flat_splice(40, 1));
    micro.append(&mut lil_vid(1));
    micro.append(&mut flat_splice(40, 1));
    micro.append(&mut lil_vid(1));
    micro.append(&mut flat_splice(40, 1));
    micro.append(&mut lil_vid(1));
    micro.append(&mut flat_splice(40, 1));
    micro.append(&mut no_mod(1));
    micro.append(&mut flat_emoji(40, 1));
    micro
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

    //let bench = bench_instructions();
    instructions.append(&mut concise_bench());
    //let bench = lil_video_micro_test();
    for _i in 0..19 {
        instructions.append(&mut concise_bench_2(None, None, None));
    }
    let mut total_frames = 0;
    for _instruction_set in instructions.iter() {
        total_frames = total_frames + _instruction_set.total_frames;
    }
    println!("total frames = {}", total_frames);

    instructions
}
