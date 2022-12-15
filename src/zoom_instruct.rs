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

fn skip_zoom(seconds: u32) -> Vec<ZoomSequence> {
    let mut skip_zoom: Vec<ZoomSequence> = Vec::new();
    skip_zoom.push(ZoomSequence {
        total_frames: 30*seconds,
        mode: ZoomMode::Zoom(ZoomInstructions {
            max_depth: 120,
            min_depth: 24,
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffere".to_string() 
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

pub fn get_zoom_a_instructions () -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    for i in 0..3 {
        instructions.append(&mut full_zoom(1));
        instructions.append(&mut scroll(29, 190));
        instructions.append(&mut full_zoom(30));
    }
    instructions
}

pub fn get_zoom_b_instructions () -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    for i in 0..3 {
        instructions.append(&mut full_zoom(15));
        instructions.append(&mut scroll(30, 190));
        instructions.append(&mut full_zoom(15));
    }
    instructions
}
