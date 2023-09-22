const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

mod block;
mod constants;
mod events;
mod utils;
mod vector;

use crate::block::{Block, BlockType};
use crate::constants::*;
use crate::vector::Vector2;
use std::collections::HashMap;
use crate::events::*;

use anyhow;
use nannou::prelude::*;
use utils::get_window_boundary;
use std::time::Instant;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let blocks = &model.blocks;

    for block in blocks {
        draw.rect()
            .xy(vec2(block.position.x, block.position.y))
            .w(block.size.x)
            .h(block.size.y)
            .color(block.color);
    }

    let spawn_distance = (model.spawn_distance * BLOCK_SIZE) * 2.0;

    let _spawn_box = draw
        .rect()
        .xy(app.mouse.position())
        .w(spawn_distance)
        .h(spawn_distance)
        .color(Rgba::new(1.0, 1.0, 1.0, 0.05))
        .stroke(WHITE)
        .stroke_weight(1.0);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)

        .mouse_pressed(mouse_pressed)
        .mouse_wheel(mouse_wheel)
        .mouse_released(mouse_released)
        .received_character(recieved_character)
        .resized(window_resized)

        .build()
        .unwrap();

    let blocks: Vec<Block> = Vec::new();
    let block_positions: HashMap<Vector2, Block> = HashMap::new();
    let last_spawned = Instant::now();
    let selected_block = BlockType::Sand;

    let window_positions = get_window_boundary(app);
    let positions_to_erase: HashMap<Vector2, Block> = HashMap::new();

    Model {
        _window,
        blocks,
        block_positions,

        last_spawned,
        spawn_distance: 5.0,
        selected_block,
        mouse_held: false,
        window_positions,
        positions_to_erase
    }
}

fn main() -> Result<(), anyhow::Error> {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .run();
    Ok(())
}
