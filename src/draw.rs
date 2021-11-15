use piston_window::{rectangle, G2d, Context};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, gr : &mut G2d) {
    let cur_x = to_coord(x);
    let cur_y = to_coord(y);

    rectangle(
        color,
        [cur_x, cur_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        gr
    );
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32
                      , con: &Context, gr: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        gr
    )
}