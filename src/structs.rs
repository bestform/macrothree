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


#[derive(Clone, Copy)]
pub struct Star {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) size: f32,
    pub(crate) brightness: f32,
}

#[derive(Clone)]
pub struct Particle {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) size: f32,
    pub(crate) color: Color,
    pub(crate) created_at: f64,
}

#[derive(Clone)]
pub struct Bullet {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) alive: bool,
    pub(crate) rot: f32,
}

pub enum PlayerMovementState {
    IDLE,
    LEFT,
    RIGHT,
    UP,
    DOWN
}

#[derive(Clone, Copy)]
pub struct Player {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) last_shot: f64,
    pub(crate) health: i32,
}

#[derive(Clone, Copy)]
pub struct PointsToAdd {
    pub(crate) amount: i32,
    pub(crate) pos: Vec2,
}

#[derive(Clone)]
pub struct FloatingMessage {
    pub(crate) message: String,
    pub(crate) shown_at: f64,
    pub(crate) pos: Vec2,
    pub(crate) scale: f32,
}
