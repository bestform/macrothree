use macroquad::prelude::screen_height;
use crate::{PARTICLE_LIFETIME, MESSAGE_LIFETIME};
use crate::game::Game;


impl Game {
    pub fn handle_lifetimes_for_frame(&mut self, frame_t: f64) {
        self.handle_bullet_lifetime();
        self.handle_particles_lifetime(frame_t);
        self.handle_enemies_lifetime();
        self.handle_stars_lifetime();
        self.handle_messages_lifetime(frame_t);
    }

    fn handle_messages_lifetime(&mut self, frame_t: f64) {
        self.messages.retain(|m| frame_t - m.shown_at < MESSAGE_LIFETIME);
    }

    fn handle_stars_lifetime(&mut self) {
        self.stars.retain(|s| s.pos.y() < screen_height());
    }

    fn handle_enemies_lifetime(&mut self) {
        self.enemies.retain(|enemy| enemy.pos.y() < screen_height() && enemy.hitpoints > 0);
    }

    fn handle_particles_lifetime(&mut self, frame_t: f64) {
        self.particles.retain(|particle| particle.pos.y() < screen_height() && frame_t - particle.created_at < PARTICLE_LIFETIME);
    }

    fn handle_bullet_lifetime(&mut self) {
        self.bullets.retain(|bullet| bullet.pos.y() > 0. && bullet.alive);
        self.enemy_bullets.retain(|bullet| bullet.pos.y() < screen_height() && bullet.alive);
    }
}
