use std::time::Duration;

pub const BLOCK_SIZE: f32 = 6.0;
pub const SPAWN_DELAY: Duration = Duration::from_millis(0);

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Direction {
    BottomLeft,
    BottomRight,
    BottomMiddle,
}
