use std::collections::HashMap;

use nannou::prelude::*;
use rand::Rng;
use uuid::Uuid;

use crate::constants::*;
use crate::vector::{Vector2, vector2};
use crate::utils::{get_blocked_directions, round_position, to_srgb, round_number};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockType {
    Sand,
    Stone,
    Erase
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockState {
    Falling,
    Stacked,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Block {
    pub position: Vector2,
    pub size: Vector2,

    velocity: Vector2,
    gravity: Vector2,

    pub color: Srgb,
    pub uid: Uuid,

    pub block_type: BlockType,
    pub block_state: BlockState,

    pub erased: bool,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            position: vector2(0.0, 0.0),
            size: vector2(BLOCK_SIZE, BLOCK_SIZE),

            velocity: vector2(0.0, 0.0),
            gravity: vector2(0.0, -8.0),

            color: Srgb::new(1.0, 1.0, 1.0),
            uid: Uuid::new_v4(),

            block_type: BlockType::Sand,
            block_state: BlockState::Falling,

            erased: false,
        }
    }
}

impl Block {
    pub fn new(block_type: BlockType, block_table: &mut Vec<Block>, block_positions: &mut HashMap<Vector2, Block>, position: Vector2) -> Self {
        let mut sand_color: Srgb = Srgb::new(1.0, 1.0, 1.0);

        let sand_colors = vec![
            *to_srgb(222, 178, 111),
            *to_srgb(229, 183, 114),
            *to_srgb(216, 172, 108),
        ];

        if block_type == BlockType::Sand {
            let random = rand::thread_rng().gen_range(0..3);
            sand_color = sand_colors[random];
        }

        let mut new_block = match block_type {
            BlockType::Sand => Self {
                color: sand_color,
                block_type: BlockType::Sand,
                ..Default::default()
            },
            BlockType::Stone => Self {
                color: *to_srgb(98, 95, 89),
                gravity: vector2(0.0, 0.0),
                block_type: BlockType::Stone,
                ..Default::default()
            },
            _ => Self {
                ..Default::default()
            },
        };
        new_block.uid = Uuid::new_v4();
        new_block.position = position;

        block_table.push(new_block);
        block_positions.insert(position, new_block);

        new_block
    }

    fn position_in_bounds(&self, position: Vector2, window_positions: &Vec<f32>) -> bool {
        let left = window_positions[0];
        let right = window_positions[1];
        let top = window_positions[2];
        let bottom = window_positions[3];

        if position.x < left || position.x > right {
            return false;
        }
        if position.y < bottom || position.y > top {
            return false;
        }

        return true;
    }

    fn apply_gravity(&mut self, block_positions: &HashMap<Vector2, Block>, window_positions: &Vec<f32>)  {
        // let mut new_velocity = vector2(0.0, 0.0);
        // let gravity_y = self.gravity.y * -1.0;

        // let mut checked_iterations = Vec::new();
        // for i in 0..gravity_y as i32 {
        //     let i = -round_number(i as f32);

        //     if checked_iterations.contains(&i) {
        //         continue;
        //     }
        //     checked_iterations.push(i);

        //     let next_position = self.position + vector2(0.0, i as f32);
        //     let block_below = block_positions.contains_key(&next_position);

        //     if block_below == false && self.position_in_bounds(next_position, window_positions) {
        //         new_velocity = vector2(0.0, i as f32);
        //     }
        // }

        // if new_velocity == vector2(0.0, 0.0) {
        //     return false;
        // }

        //self.velocity = new_velocity;
        // self.velocity = self.gravity;
        let new_velocity = self.gravity;
        // let next_position = round_position(self.position + new_velocity);


        self.velocity = new_velocity;
    }

    fn stack_block(&mut self, block_positions: &HashMap<Vector2, Block>) {
        let blocked_directions = get_blocked_directions(*self, &block_positions);

        // let left_open = block_positions.contains_key(&(self.position + vector2(-BLOCK_SIZE, 0.0))) == false;
        // let right_open = block_positions.contains_key(&(self.position + vector2(BLOCK_SIZE, 0.0))) == false;

        let left_open = blocked_directions.contains_key(&Direction::BottomLeft) == false;
        let right_open = blocked_directions.contains_key(&Direction::BottomRight) == false;
        let middle_open = blocked_directions.contains_key(&Direction::BottomMiddle) == false;

        if middle_open || (left_open == false && right_open == false) {
            return;
        }

        let move_direction = if left_open && right_open {
            let random = random_f32();
            if random > 0.5 {
                BLOCK_SIZE
            } else {
                BLOCK_SIZE
            }
        } else if left_open {
            -BLOCK_SIZE
        } else if right_open {
            BLOCK_SIZE
        } else {
            return;
        };

        // let new_position = round_position(self.position + vector2(move_direction, -BLOCK_SIZE));

        // if self.position_in_bounds(new_position, window_positions) == false && block_positions.contains_key(&new_position) == false {
        //     return;
        // }
        

        // self.position = new_position;
        self.velocity = vector2(move_direction, -BLOCK_SIZE);
    }

    pub fn update(&mut self, _app: &App, block_positions: &HashMap<Vector2, Block>, window_positions: &Vec<f32>) {
        if self.block_type == BlockType::Stone { // Stone blocks don't move
            self.position = round_position(self.position); 
            return;
        }

        self.velocity = vector2(0.0, 0.0);

        let position_below = self.position + vector2(0.0, -BLOCK_SIZE);

        let block_below = match block_positions.get(&position_below) {
            Some(block) => *block,
            None => {
                self.apply_gravity(block_positions, window_positions);

                let next_position = round_position(self.position + self.velocity);

                if block_positions.contains_key(&next_position) || self.position_in_bounds(next_position, window_positions) == false {
                    self.block_state = BlockState::Stacked;
                    return;
                }

                self.block_state = BlockState::Falling;
                self.position = next_position;
                return;
            },
        };

        if block_below.block_state == BlockState::Stacked {
            self.stack_block(&block_positions);
        }

        let next_position = round_position(self.position + self.velocity);

        if block_positions.contains_key(&next_position) || self.position_in_bounds(next_position, window_positions) == false {
            return;
        }

        self.block_state = BlockState::Stacked; // only will happen when gravity isnt applied
        self.position = next_position;
    }
}
