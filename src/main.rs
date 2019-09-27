use std::path::Path;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};

fn main() {
    let path = Path::new(&"result.png");

    let mut image = RgbImage::new(15, 30);

    let font = Vec::from(include_bytes!("../unifont/unifont-12.1.03.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    let height = 30.0;
    let scale = Scale {
        x: height,
        y: height,
    };
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
        "µ∆ƒº§¢£∞§",
    );

    let _ = image.save(path).unwrap();
}