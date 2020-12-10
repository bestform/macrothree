use macroquad::prelude::screen_height;
use crate::{Star, Bullet, Particle, PARTICLE_LIFETIME, MESSAGE_LIFETIME};
use crate::structs::{Enemy, FloatingMessage};

pub fn handle_stars_lifetime(stars: &mut Vec<Star>) {
    stars.retain(|s| s.pos.y() < screen_height());
}

pub fn handle_messages_lifetime(messages: &mut Vec<FloatingMessage>, frame_t: f64) {
    messages.retain(|m| frame_t - m.shown_at < MESSAGE_LIFETIME);
}

pub fn handle_bullet_lifetime(bullets: &mut Vec<Bullet>, enemy_bullets: &mut Vec<Bullet>) {
    bullets.retain(|bullet| bullet.pos.y() > 0. && bullet.alive);
    enemy_bullets.retain(|bullet| bullet.pos.y() < screen_height() && bullet.alive);
}

pub fn handle_particles_lifetime(particles: &mut Vec<Particle>, frame_t: f64) {
    particles.retain(|particle| particle.pos.y() < screen_height() && frame_t - particle.created_at < PARTICLE_LIFETIME);
}

pub fn handle_enemies_lifetime(enemies: &mut Vec<Enemy>) {
    enemies.retain(|enemy| enemy.pos.y() < screen_height() && enemy.hitpoints > 0);
}

