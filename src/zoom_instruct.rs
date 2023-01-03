#[derive(Clone)]
pub struct ZoomInstructions {
    pub max_depth: u32,
    pub min_depth: u32,
    pub lil_imgs_dir: String
}

pub struct ScrollInstructions {
    pub direction: (f32, f32),
    pub velocity: f32,
    pub depth: i32,
    pub lil_imgs_dir: String
}

pub enum ZoomMode {
    Zoom(ZoomInstructions),
    Scroll(ScrollInstructions)
}

pub struct ZoomSequence {
    pub total_frames: u32,
    pub mode: ZoomMode
}

fn full_zoom(seconds: u32) -> Vec<ZoomSequence> {
    let mut full_zoom: Vec<ZoomSequence> = Vec::new();
    full_zoom.push(ZoomSequence {
        total_frames: 30 * seconds,
        mode: ZoomMode::Zoom(ZoomInstructions { 
            max_depth: 200, 
            min_depth: 4, 
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffere".to_string() 
        })
    });
    full_zoom
}

fn skip_zoom(seconds: u32, min_depth: u32, max_depth: u32) -> Vec<ZoomSequence> {
    let mut skip_zoom: Vec<ZoomSequence> = Vec::new();
    skip_zoom.push(ZoomSequence {
        total_frames: 30*seconds,
        mode: ZoomMode::Zoom(ZoomInstructions {
            max_depth,
            min_depth,
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string() 
        })
    });
    skip_zoom
}

fn scroll(seconds: u32, depth: i32) -> Vec<ZoomSequence> {
    let mut scroll: Vec<ZoomSequence> = Vec::new();
    scroll.push(ZoomSequence {
        total_frames: seconds*30,
        mode: ZoomMode::Scroll(ScrollInstructions {
            direction: (1.0, 1.0),
            velocity: 12.0,
            depth,
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string()
        })
    });
    scroll
}

pub fn get_zoom_a_instructions (minutes: u8) -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    for i in 0..minutes {
        instructions.append(&mut full_zoom(3));
        instructions.append(&mut skip_zoom(9, 24, 40));
        instructions.append(&mut full_zoom(3));

        instructions.append(&mut scroll(13, 1000));
        instructions.append(&mut skip_zoom(4, 60, 120));
        instructions.append(&mut scroll(13, 60));

        instructions.append(&mut full_zoom(18));
    }
    let total_frames = instructions.iter().fold(0, |acc, x| acc + x.total_frames);
    println!("Total frames in zoom_a_instructions: {}", total_frames);
    instructions
}

pub fn get_zoom_b_instructions (minutes: u8) -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    for i in 0..minutes {
        instructions.append(&mut full_zoom(3));
        instructions.append(&mut skip_zoom(9, 40, 60));
        instructions.append(&mut full_zoom(18));
        instructions.append(&mut scroll(13, 400));
        instructions.append(&mut skip_zoom(4, 15, 60));
        instructions.append(&mut scroll(16, 190));
    }
    let total_frames = instructions.iter().fold(0, |acc, x| acc + x.total_frames);
    println!("Total frames in zoom_b_instructions: {}", total_frames);
    instructions
}
