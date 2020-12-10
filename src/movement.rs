use macroquad::prelude::*;
use crate::{Particle, Star, Bullet, Player, PLAYER_SPEED, TOP_MARGIN, BOTTOM_MARGIN, PLAYER_SIZE, PlayerMovementState};
use crate::math::{clampx, clampy};
use crate::input::handle_player_input;

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

    let player_movement_states = handle_player_input();

    for state in player_movement_states {
        match state {
            PlayerMovementState::IDLE => { player.vel = Vec2::new(0., 0.); }
            PlayerMovementState::LEFT => { player.vel.set_x(-1. * PLAYER_SPEED); }
            PlayerMovementState::RIGHT => { player.vel.set_x(PLAYER_SPEED); }
            PlayerMovementState::UP => { player.vel.set_y(-1. * PLAYER_SPEED); }
            PlayerMovementState::DOWN => { player.vel.set_y(PLAYER_SPEED); }
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