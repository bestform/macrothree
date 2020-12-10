use crate::{Star, Bullet, Particle};
use macroquad::prelude::*;
use crate::structs::Enemy;

pub fn draw_debug(stars: Vec<Star>, bullets: Vec<Bullet>, enemy_bullets: Vec<Bullet>, particles: Vec<Particle>, enemies: Vec<Enemy>) {
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
        &std::fmt::format(format_args!("Enemy Bullets: {}", enemy_bullets.len())),
        10.,
        60.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("Particles: {}", particles.len())),
        10.,
        80.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("Enemies: {}", enemies.len())),
        10.,
        100.,
        20.,
        WHITE,
    );
    draw_text(
        &std::fmt::format(format_args!("FPS: {}", get_fps())),
        10.,
        120.,
        20.,
        WHITE,
    );
}