use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
pub fn vector2(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
}

impl Hash for Vector2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Eq for Vector2 {}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// impl Vector2 {
//     pub fn distance(self, other: Vector2) -> f32 {
//         let x = self.x - other.x;
//         let y = self.y - other.y;

//         return (x * x + y * y).sqrt();
//     }
//     pub fn add(self, other: Vector2) -> Vector2 {
//         vector2(self.x + other.x, self.y + other.y)
//     }
// }