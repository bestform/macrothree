use macroquad::prelude::*;
use crate::{Particle, Star, Bullet, Player, PLAYER_SPEED, TOP_MARGIN, BOTTOM_MARGIN, PLAYER_SIZE};
use crate::math::{clampx, clampy};

pub fn handle_particles_move(particles: &mut Vec<Particle>) {
    for particle in particles {
        particle.pos += particle.vel;
    }
}

pub fn handle_stars_move(stars: &mut Vec<Star>) {
    for star in stars {
        star.pos += star.vel;
    }
}

pub fn handle_bullet_move(bullets: &mut Vec<Bullet>) {
    for bullet in bullets {
        bullet.pos += bullet.vel;
    }
}

pub fn handle_player_movement(player: &mut Player) {
    player.vel = Vec2::new(0., 0.);

    if !(is_key_down(KeyCode::A) && is_key_down(KeyCode::D)) {
        if is_key_down(KeyCode::A) {
            player.vel.set_x(-1. * PLAYER_SPEED);
        }
        if is_key_down(KeyCode::D) {
            player.vel.set_x(PLAYER_SPEED);
        }
    }
    if !(is_key_down(KeyCode::W) && is_key_down(KeyCode::S)) {
        if is_key_down(KeyCode::W) {
            player.vel.set_y(-1. * PLAYER_SPEED);
        }
        if is_key_down(KeyCode::S) {
            player.vel.set_y(PLAYER_SPEED);
        }
    }

    if player.vel.length() == 0. {
        return;
    }

    player.vel = player.vel.normalize() * PLAYER_SPEED;
    player.pos += player.vel;
    player.pos = clampx(player.pos, 0., screen_width());
    player.pos = clampy(player.pos, TOP_MARGIN, screen_height() - PLAYER_SIZE - BOTTOM_MARGIN);
}