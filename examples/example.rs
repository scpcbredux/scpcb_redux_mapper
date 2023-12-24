use std::path::PathBuf;

use clap::Parser;
use image::{ImageBuffer, Rgb, RgbImage};
use image_colored_text::{
    draw::single_line::{draw_line_of_colored_text, DrawCoord},
    ttp::TextToPrint,
};
use rusttype::{Font, Scale};
use scpcb_redux_mapper::*;

const FONT_WIDTH: f32 = 10.0;
const FONT_HEIGHT: f32 = 10.0;
const BLOCK_SIZE: u32 = 91;
const DRAWING_GRAPHIC_FONT: &'static [u8] = include_bytes!("DejaVuSansMono.ttf");

/// Simple example to generate a map
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed to generate the map
    #[arg(short, long, default_value = "anjay")]
    seed: String,
    /// Export path to map
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut map = Map::new_from_string(20, 20, &args.seed);
    while map.room_1_amount < 2 {
        map.generate(24);
    }

    let mut img_buf: RgbImage = ImageBuffer::from_pixel(
        map.width * BLOCK_SIZE,
        map.height * BLOCK_SIZE,
        Rgb([150, 150, 150]),
    );

    let mut cur_room_1 = 0;
    let mut cur_room_2 = 0;
    let mut cur_room_2c = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            if let Some(room) = map.room_array[x as usize][y as usize] {
                match room.kind {
                    RoomType::Room1 => match cur_room_1 {
                        0 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 255], "rz_exit_1_gatea");
                            cur_room_1 += 1;
                        }
                        1 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 255], "rz_exit_1_gateb");
                            cur_room_1 += 1;
                        }
                        _ => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 255], "rz_room_1_endroom");
                        }
                    },
                    RoomType::Room2 => match cur_room_2 {
                        0 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2_offices");
                            cur_room_2 += 1;
                        }
                        1 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2_offices_2");
                            cur_room_2 += 1;
                        }
                        2 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2_poffices");
                            cur_room_2 += 1;
                        }
                        3 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2_toilets");
                            cur_room_2 += 1;
                        }
                        4 => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2_medibay");
                            cur_room_2 += 1;
                        }
                        _ => {
                            set_pixels(&mut img_buf, x, y, [255, 255, 50], "rz_room_2");
                        }
                    },
                    RoomType::Room2C => {
                        if cur_room_2c == 0 {
                            set_pixels(&mut img_buf, x, y, [0, 200, 0], "rz_room_2c_ec");
                            cur_room_2c += 1;
                        } else {
                            set_pixels(&mut img_buf, x, y, [0, 200, 0], "rz_room_2c");
                        }
                    }
                    RoomType::Room3 => {
                        set_pixels(&mut img_buf, x, y, [50, 255, 255], "rz_room_3");
                    }
                    RoomType::Room4 => {
                        set_pixels(&mut img_buf, x, y, [100, 100, 255], "rz_room_4");
                    }
                }
            }
        }
    }

    let save_path = args.path.join(format!("{}.jpg", args.seed));
    img_buf.save(save_path).unwrap();
}

fn set_pixels(img_buf: &mut RgbImage, x: u32, y: u32, color: [u8; 3], room_name: &str) {
    let x_scaled = x * BLOCK_SIZE;
    let y_scaled = y * BLOCK_SIZE;

    for x in x_scaled..(x_scaled + BLOCK_SIZE) {
        for y in y_scaled..(y_scaled + BLOCK_SIZE) {
            let pixel = img_buf.get_pixel_mut(x, y);
            *pixel = Rgb(color);
        }
    }

    let font = Font::try_from_bytes(DRAWING_GRAPHIC_FONT).unwrap();
    let scale = Scale {
        x: FONT_WIDTH,
        y: FONT_HEIGHT,
    };
    let to_print = vec![TextToPrint::new(room_name.to_string(), Rgb([0, 0, 0]))];

    let xpos = DrawCoord::StartingAt(x_scaled as f32);
    let ypos = DrawCoord::StartingAt(y_scaled as f32);
    draw_line_of_colored_text(img_buf, &xpos, &ypos, &to_print, &font, &scale);
}
