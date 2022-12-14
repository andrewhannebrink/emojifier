#[derive(Clone)]
pub struct ZoomInstructions {
    pub max_depth: u32,
    pub min_depth: u32,
    pub lil_imgs_dir: String
}

pub struct ScrollInstructions {
    pub direction: (f32, f32),
    pub lil_imgs_dir: Option<String>
}

pub enum ZoomMode {
    Zoom(ZoomInstructions),
    Scroll(ScrollInstructions)
}

pub struct ZoomSequence {
    pub total_frames: u32,
    pub mode: ZoomMode
}

pub fn get_zoom_a_instructions () -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    instructions.push(ZoomSequence {
        total_frames: 200,
        mode: ZoomMode::Zoom(ZoomInstructions {
            max_depth: 24,
            min_depth: 4,
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string()
        })
    });
//  instructions.push(ZoomSequence {
//      total_frames: 200,
//      mode: ZoomMode::Scroll(ScrollInstructions {
//          direction: (1.0, 0_f32),
//          lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string()
//      })
//  });
    instructions
}

pub fn get_zoom_b_instructions () -> Vec<ZoomSequence> {
    let mut instructions: Vec<ZoomSequence> = Vec::new();
    instructions.push(ZoomSequence {
        total_frames: 200,
        mode: ZoomMode::Zoom(ZoomInstructions {
            max_depth: 60,
            min_depth: 20,
            lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string()
        })
    });
//  instructions.push(ZoomSequence {
//      total_frames: 200,
//      mode: ZoomMode::Scroll(ScrollInstructions {
//          direction: (1.0, 0_f32),
//          lil_imgs_dir: "io/lil_imgs/emoji_big_buffered".to_string()
//      })
//  });
    instructions
}
