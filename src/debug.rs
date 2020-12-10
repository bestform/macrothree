use crate::{Star, Bullet};
use macroquad::prelude::*;

pub fn draw_debug(stars: Vec<Star>, bullets: Vec<Bullet>) {
    draw_text(
        &std::fmt::format(format_args!("Stars: {}", stars.len())),
        10.,
        20.,
        20.,
        WHITE
    );
    draw_text(
        &std::fmt::format(format_args!("Bullets: {}", bullets.len())),
        10.,
        40.,
        20.,
        WHITE
    );
}