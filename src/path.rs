static INPUT_DIR: &str = "io/input/";
static OUTPUT_DIR: &str = "io/output/";
static INT_ARRAY: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
pub static QUADRANT_A: Quadrant = Quadrant::new(QuadrantEnum::A);
pub static QUADRANT_B: Quadrant = Quadrant::new(QuadrantEnum::A);

pub enum QuadrantEnum {
    A, B
}

pub struct Quadrant<'a> {
    quadrant_enum: QuadrantEnum,
    dir: &'a str
}

impl Quadrant<'_> {
    pub const fn new(quadrant_enum: QuadrantEnum) -> Self {
        match quadrant_enum {
            QuadrantEnum::A => { Self { quadrant_enum, dir: "a/" } },
            QuadrantEnum::B => { Self { quadrant_enum, dir: "b/" } }
        }
    }
}

pub fn prepend_zeroes(n: i32) -> String {
    let mut number_str = String::new();
    for b in 0..5 {
        let digit = (n as i32 / 10_i32.pow(b)) % 10;
        number_str = [INT_ARRAY[digit as usize].to_string(), number_str].concat();
    }
    number_str
}

pub fn input_dir(quadrant: &Quadrant) -> String {
    [INPUT_DIR, quadrant.dir].concat()
}
pub fn output_dir(quadrant: &Quadrant) -> String {
    [OUTPUT_DIR, quadrant.dir].concat()
}

pub fn input_path(quadrant: &Quadrant, frame_number_str: &String) -> String {
    [
        INPUT_DIR,
        quadrant.dir,
        frame_number_str,
        ".jpeg"
    ].concat()
}
pub fn output_path(quadrant: &Quadrant, frame_number_str: &String) -> String {
    [
        OUTPUT_DIR,
        quadrant.dir,
        frame_number_str,
        ".jpeg"
    ].concat()
}
