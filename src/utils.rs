use image::{Rgb, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_line_segment_mut};
use std::vec;

use super::{HEIGHT, WIDTH};
use crate::file_loader::*;

pub fn rgba8_to_rgb8(
    input: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];

    let mut i = 0;
    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for chunk in input.chunks(4) {
        // ... and copy each of them to output, leaving out the A byte
        output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
        i += 3;
    }

    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}

// TODO: Set scale as input arg
const SCALE: f32 = 20.0;

pub fn draw_blocks(frame: &mut RgbaImage, blocks: &vec::Vec<LocationScale>) {
    let blockcol = Rgba([255, 0, 0, 255]);
    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;

    for block in blocks.iter() {
        let start_x = center_x as f32 + (block.location.x - (block.scale.x / 2.0)) * SCALE;
        let start_y = center_y as f32 + (block.location.y - (block.scale.y / 2.0)) * SCALE;
        let end_x = center_x as f32 + (block.location.x + (block.scale.x / 2.0)) * SCALE;
        let end_y = center_y as f32 + (block.location.y + (block.scale.y / 2.0)) * SCALE;

        draw_line_segment_mut(frame, (start_x, start_y), (end_x, start_y), blockcol);
        draw_line_segment_mut(frame, (end_x, start_y), (end_x, end_y), blockcol);
        draw_line_segment_mut(frame, (end_x, end_y), (start_x, end_y), blockcol);
        draw_line_segment_mut(frame, (start_x, end_y), (start_x, start_y), blockcol);
    }
}

pub fn draw_user(frame: &mut RgbaImage, user: &UserLoc, usercol: Rgba<u8>) {
    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;

    let user_x = (user.x * SCALE) as i32 + center_x as i32;
    let user_y = (user.y * SCALE) as i32 + center_y as i32;
    draw_filled_ellipse_mut(
        frame,
        (user_x, user_y),
        (0.5 * SCALE) as i32,
        (0.5 * SCALE) as i32,
        usercol,
    );
}
