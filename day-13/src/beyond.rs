use crate::computer::Program;
use crate::arcade::{ArcdeCabinet, Position};

use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;

use std::fs::read_to_string;
use std::convert::TryInto;

fn get_dims(arcade: &ArcdeCabinet) -> (i64, i64, i64, i64) {
    let screen = &arcade.get_screen();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    for p in screen.keys() {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    return (min_x, min_y, max_x, max_y);
}

fn build_frame_data(arcade: &ArcdeCabinet, dims: (i64, i64, i64, i64)) -> Vec<u8> {
    let (min_x, min_y, max_x, max_y) = dims;

    let mut frame_data = Vec::new();

    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if let Some(t) = arcade.get_screen().get(&Position::new(x, y)) {
                frame_data.push(*t as u8);
            } else {
                panic!("Unknown Point");
            }
        }
    }

    return frame_data;
}

pub fn beyond() {
    let mut intcode_str = read_to_string("input/input.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = Program::new_from_str(intcode_str);

    let mut arcade = ArcdeCabinet::new(program);
    arcade.run();

    let dims = get_dims(&arcade);
    let (min_x, min_y, max_x, max_y) = dims;

    println!("Dim: {} x {} -> {} x {}", min_x, min_y, max_x, max_y);

    let width: u16 = ((max_x+1)-min_x).try_into().unwrap();
    let height: u16 = ((max_y+1)-min_y).try_into().unwrap();
    let color_map = [
                        0x00, 0x00, 0x00,   // black for background
                        0xFF, 0xFF, 0xFF,   // white for border
                        0xFF, 0x00, 0x00,   // red for blocks
                        0x00, 0xFF, 0x00,   // green for paddle
                        0x00, 0x00, 0xFF    // blue for ball
                    ];

    let mut image = File::create("game.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, &color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    arcade.reset();
    arcade.insert_coins();

    let mut step = 0;
    while !arcade.step() {
        if step % 500 == 0 {
            let data = build_frame_data(&arcade, dims);
            let mut frame = Frame::default();
            frame.width = width;
            frame.height = height;
            frame.buffer = Cow::Borrowed(data.as_slice());
            encoder.write_frame(&frame).unwrap();
        }
        step += 1;
    }
}
