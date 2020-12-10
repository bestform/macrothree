use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) hitpoints: i32,
    pub(crate) tex_idx: usize,
    pub(crate) last_shot: f64,
    pub(crate) shot_freq: f64,
}
