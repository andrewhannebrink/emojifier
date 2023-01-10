use async_process::Stdio;
use crate::path;

pub async fn make_video_from_dir(dir: &String, dest_path: &String) {
    //let ffmpegCmd = "ffmpeg -r 30 -i io/output/b/%05d.jpeg -vb 20000k -c:v libx264 -pix_fmt yuv420p io/output/vid/20min-b3.mp4"
    async_process::Command::new("ffmpeg")
            .arg("-y")
            .args(&["-r", "30"])
            .args(&["-i", &path::output_path(&path::QUADRANT_B, &"%05d".to_string())])
            .args(&["-vb", "20000k"])
            .args(&["-c:v", "libx264"])
            .args(&["-pix_fmt", "yuv420p"])
            .arg([
                "io/output/vid",
                "auto_op.mp4"
            ].join("/"))
            .spawn();
}
