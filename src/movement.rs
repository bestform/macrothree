use macroquad::prelude::*;
use crate::{Particle, Star, Bullet, Player, PLAYER_SPEED, TOP_MARGIN, BOTTOM_MARGIN, PLAYER_SIZE, PlayerMovementState};
use crate::math::{clampx, clampy};
use crate::structs::Enemy;

pub fn handle_particles_move(particles: &mut Vec<Particle>) {
    for particle in particles {
        particle.pos += particle.vel;
    }
}

pub fn handle_stars_move(stars: &mut Vec<Star>, player_movement_states: Vec<PlayerMovementState>) {
    let mut parallax = 0.;
    for state in &player_movement_states {
        match state {
            PlayerMovementState::IDLE => {}
            PlayerMovementState::LEFT => {
                parallax = 0.5;
            }
            PlayerMovementState::RIGHT => {
                parallax = -0.5;
            }
            PlayerMovementState::UP => {}
            PlayerMovementState::DOWN => {}
        }
    }
    for star in stars {
        star.pos += star.vel;
        star.pos.set_x(star.pos.x() + star.brightness * parallax)

    }

}

pub fn handle_enemies_move(enemies: &mut Vec<Enemy>) {
    for enemy in enemies {
        enemy.pos += enemy.vel;
    }
}

pub fn handle_bullets_move(bullets: &mut Vec<Bullet>, enemy_bullets: &mut Vec<Bullet>) {
    for bullet in bullets {
        bullet.pos += bullet.vel;
    }
    for bullet in enemy_bullets {
        bullet.pos += bullet.vel;
        bullet.rot += 0.1;
    }
}

pub fn handle_player_movement(player: &mut Player, player_movement_states: Vec<PlayerMovementState>) {

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