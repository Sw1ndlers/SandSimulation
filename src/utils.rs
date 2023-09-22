use std::collections::HashMap;

use nannou::prelude::*;
use rand::Rng;

use crate::block::Block;
use crate::constants::*;
use crate::vector::{vector2, Vector2};

pub fn to_srgb(r: u8, g: u8, b: u8) -> Srgba {
    Srgba::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

pub fn generate_positions(distance: f32, randomize: bool) -> Vec<Vector2> {
    let mut rng = rand::thread_rng();
    let mut positions: Vec<Vector2> = Vec::new();

    let num_positions = (distance * 2.0) as usize;

    if randomize == false {
        for x in -distance as i32..=distance as i32 {
            for y in -distance as i32..=distance as i32 {
                positions.push(vector2(x as f32, y as f32) * BLOCK_SIZE);
            }
        }

        return positions;
    }

    while positions.len() < num_positions {
        let mut position = vector2(
            rng.gen_range(-distance..=distance),
            rng.gen_range(-distance..=distance),
        );
        position = position * BLOCK_SIZE;

        if positions.contains(&position) == false {
            positions.push(position);
        }
    }

    positions
}

// pub fn get_screen_grid(app: &App) -> Vec<Vector2> {
//     let window = app.window_rect();
//     let mut grid: Vec<Vector2> = Vec::new();

//     let mut x = window.left();
//     let mut y = window.bottom();

//     while x < window.right() {
//         while y < window.top() {
//             grid.push(vector2(x, y));
//             y += BLOCK_SIZE;
//         }
//         y = window.bottom();
//         x += BLOCK_SIZE;
//     }

//     grid
// }

pub fn get_blocked_directions(self_block: Block,blocks_table: &HashMap<Vector2, Block>) -> HashMap<Direction, Block> {
    let mut blocked_directions: HashMap<Direction, Block> = HashMap::new();

    let self_position = self_block.position;
    let block_size = BLOCK_SIZE;

    let bottom_left_position = self_position + vector2(-block_size, -block_size);
    let bottom_right_position = self_position + vector2(block_size, -block_size);
    let bottom_middle_position = self_position + vector2(0.0, -block_size);

    if let Some(bottom_middle_block) = blocks_table.get(&bottom_middle_position) {
        blocked_directions.insert(Direction::BottomMiddle, *bottom_middle_block);

        if let Some(bottom_left_block) = blocks_table.get(&bottom_left_position) {
            blocked_directions.insert(Direction::BottomLeft, *bottom_left_block);
        }

        if let Some(bottom_right_block) = blocks_table.get(&bottom_right_position) {
            blocked_directions.insert(Direction::BottomRight, *bottom_right_block);
        }
    }

    blocked_directions
}

pub fn round_position(position: Vector2) -> Vector2 {
    vector2(
        (position.x / BLOCK_SIZE).round() * BLOCK_SIZE,
        (position.y / BLOCK_SIZE).round() * BLOCK_SIZE,
    )
}

pub fn round_number(number: f32) -> f32 {
    (number / BLOCK_SIZE).round() * BLOCK_SIZE
}

pub fn get_window_boundary(app: &App) -> Vec<f32> {
    let boundary = app.window_rect();

    // let mut window_positions = HashMap::new();
    // window_positions.insert(Direction::Left, boundary.left());
    // window_positions.insert(Direction::Right, boundary.right());
    // window_positions.insert(Direction::Top, boundary.top());
    // window_positions.insert(Direction::Bottom, boundary.bottom());

    let mut window_positions: Vec<f32> = Vec::new();
    window_positions.push(boundary.left());
    window_positions.push(boundary.right());
    window_positions.push(boundary.top());
    window_positions.push(boundary.bottom());

    window_positions
}
