use macroquad::prelude::screen_height;
use crate::{Star, Bullet, Particle, PARTICLE_LIFETIME};

pub fn handle_stars_lifetime(stars: &mut Vec<Star>) {
    stars.retain(|s| s.pos.y() < screen_height());
}

pub fn handle_bullet_lifetime(bullets: &mut Vec<Bullet>) {
    bullets.retain(|bullet| bullet.pos.y() > 0.);
}

pub fn handle_particles_lifetime(particles: &mut Vec<Particle>, frame_t: f64) {
    particles.retain(|particle| particle.pos.y() < screen_height() && frame_t - particle.created_at < PARTICLE_LIFETIME);
}
