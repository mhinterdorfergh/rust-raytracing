use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    /*
     * PPM File:
     * P3
     * <image_width> <image_height>
     * <max_color=255>
     * r g b  r g b  ...  r g b
     * ...
     * r g b  r g b  ...  r g b
     */

    // io
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .expect("Unable to open file");
    let mut writer = BufWriter::new(file);

    dotenv::dotenv().ok();
    env_logger::init();

    // print header
    writer
        .write_all(
            format!(
                r###"P3
{image_width} {image_height}
{max_color}
"###,
                image_width = IMAGE_WIDTH,
                image_height = IMAGE_HEIGHT,
                max_color = 255,
            )
            .as_bytes(),
        )
        .expect("Unable to write data");

    log::debug!("wrote header");

    // draw line by line from top to bottom
    for j in (0..IMAGE_HEIGHT).rev() {
        // from left to right
        for i in 0..IMAGE_WIDTH {
            let r = ((i as f64) / ((IMAGE_WIDTH - 1) as f64)) as f64;
            let g = ((j as f64) / ((IMAGE_HEIGHT - 1) as f64)) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writer
                .write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .expect("failed writing data");
            log::debug!(
                "wrote data {} / {}",
                (IMAGE_HEIGHT - j - 1) * IMAGE_WIDTH + i + 1,
                IMAGE_HEIGHT * IMAGE_WIDTH
            );
        }
    }
}
