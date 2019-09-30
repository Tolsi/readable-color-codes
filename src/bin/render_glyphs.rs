use std::fs::{create_dir, File, read_dir};
use std::io::prelude::*;
use std::path::Path;

use image::*;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use md5::compute;
use rayon::prelude::*;
use rusttype::{FontCollection, Scale};

fn main() {
    read_dir("unifont/").unwrap().for_each(|r| {
        let font_path = r.unwrap().path();
        let result_folder_name = font_path.file_name().unwrap().to_str().unwrap().replace(".", "_");
        let _ = create_dir(format!("{}/{}", "result", result_folder_name));
        let f: Vec<u8> = File::open(font_path.clone()).unwrap().bytes().map(|b| b.unwrap()).collect();
        let font = FontCollection::from_bytes(f)
            .unwrap()
            .into_font()
            .unwrap();

        let height = 30.0;
        let scale = Scale {
            x: height,
            y: height,
        };

        (('\u{0000}' as u32)..('\u{FFFF}' as u32)).into_par_iter().for_each(|i| {
            let mut image = RgbImage::new(30, 30);

            let c_result = std::char::from_u32(i);
            if c_result.is_some() {
                let c = c_result.unwrap();

                draw_filled_rect_mut(
                    &mut image,
                    Rect::at(0, 0).of_size(270, 30),
                    Rgb([255u8, 255u8, 255u8]),
                );

                draw_text_mut(
                    &mut image,
                    Rgb([0u8, 0u8, 0u8]),
                    0,
                    0,
                    scale,
                    &font,
                    &c.to_string(),
                );

                let image_md5 = compute(image.clone().into_raw());

                let _ = image.save(Path::new(&format!("result/{:x}.png", image_md5))).unwrap();
            }
        })
    });
}