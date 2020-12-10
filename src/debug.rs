use crate::{Star, Bullet, Particle};
use macroquad::prelude::*;

pub fn draw_debug(stars: Vec<Star>, bullets: Vec<Bullet>, particles: Vec<Particle>) {
    draw_text(
        &std::fmt::format(format_args!("Stars: {}", stars.len())),
        10.,
        20.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("Bullets: {}", bullets.len())),
        10.,
        40.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("Particles: {}", particles.len())),
        10.,
        60.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("FPS: {}", get_fps())),
        10.,
        80.,
        20.,
        WHITE,
    );
}