use crate::block::{Block, BlockType};
use crate::constants::*;
use crate::utils::*;
use crate::vector::{Vector2, vector2};

use nannou::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct Model {
    pub _window: WindowId,
    pub blocks: Vec<Block>,
    pub block_positions: HashMap<Vector2, Block>,

    pub last_spawned: Instant,
    pub spawn_distance: f32,

    pub selected_block: BlockType,
    pub mouse_held: bool,
    pub window_positions: Vec<f32>,

    pub positions_to_erase: HashMap<Vector2, Block>,

}

fn cooldown_over(model: &mut Model) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(model.last_spawned);

    if elapsed < SPAWN_DELAY {
        return false;
    }

    model.last_spawned = now;
    true
}

fn handle_mousedown(app: &App, model: &mut Model) {
    let mouse_position = app.mouse.position();
    let mouse_position = vector2(mouse_position.x, mouse_position.y);

    let randomize_positions = match model.selected_block {
        BlockType::Sand => true,
        BlockType::Stone => false,
        BlockType::Erase => false,
    };
    let mut positions = generate_positions(model.spawn_distance, randomize_positions);

    if model.spawn_distance == 1.0 { // Spawn a single block
        positions = vec![vector2(0.0, 0.0)];
    }

    for position in positions {
        let block_position = mouse_position + position;
        let block_position = round_position(block_position);

        if model.selected_block == BlockType::Erase {
            match model.block_positions.get(&block_position) {
                Some(block) => {
                    model.positions_to_erase.insert(block_position, *block);
                }
                None => {}
            }
            continue;
        }


        if model.block_positions.contains_key(&block_position) {
            let block = model.block_positions.get_mut(&block_position).unwrap();
            block.color = *to_srgb(0, 0, 255);

            continue;
        }

        Block::new(
            model.selected_block,
            &mut model.blocks,
            &mut model.block_positions,
            block_position,
        );
    }
}

pub fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_held = true;
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_held = false;
}

pub fn mouse_wheel(_app: &App, model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {
    match _dt {
        MouseScrollDelta::LineDelta(_x, y) => {
            let new_distance = model.spawn_distance + y;
            if new_distance < 1.0 {
                model.spawn_distance = 1.0;
                return;
            }
            model.spawn_distance = new_distance;
        }
        MouseScrollDelta::PixelDelta(_position) => {}
    }
}

pub fn recieved_character(_app: &App, model: &mut Model, char: char) {
    match char {
        '1' => model.selected_block = BlockType::Sand,
        '2' => model.selected_block = BlockType::Stone,
        '3' => model.selected_block = BlockType::Erase,
        _ => {}
    }
}

pub fn window_resized(_app: &App, model: &mut Model, _dim: Vec2) {
    model.window_positions = get_window_boundary(_app);
}

pub fn update(app: &App, model: &mut Model, _update: Update) {

    if model.mouse_held {
        if !cooldown_over(model) {
            return;
        }

        handle_mousedown(app, model);
    }

    let blocks = &mut model.blocks;
    let mut new_blocks: Vec<Block> = Vec::new();
    let mut new_block_positions: HashMap<Vector2, Block> = HashMap::new();

    for block in blocks.iter_mut() {
        if let Some(_) = model.positions_to_erase.get(&block.position) {
            continue; // dont insert into new_blocks
        }

        block.update(app, &new_block_positions, &model.window_positions);

        new_block_positions.insert(block.position, *block);
        new_blocks.push(*block);
    }

    model.positions_to_erase = HashMap::new();
    model.blocks = new_blocks;
    model.block_positions = new_block_positions;
}
