use macroquad::prelude::*;

use crate::game::Game;

impl Game {
    pub fn handle_movement_for_frame(&mut self) {
        self.handle_enemies_move();
        self.handle_bullets_move();
        self.handle_particles_move();
        self.handle_messages_move();
        self.handle_stars_move();
    }

    fn handle_stars_move(&mut self) {
        for star in self.stars.iter_mut() {
            star.pos += star.vel;
        }
    }

    fn handle_messages_move(&mut self) {
        for m in self.messages.iter_mut() {
            m.pos += Vec2::new(0., -2.);
        }
    }

    fn handle_particles_move(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.pos += particle.vel;
        }
    }

    fn handle_enemies_move(&mut self) {
        for enemy in self.enemies.iter_mut() {
            enemy.pos += enemy.vel;
        }
    }

    fn handle_bullets_move(&mut self) {
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }
        for bullet in self.enemy_bullets.iter_mut() {
            bullet.pos += bullet.vel;
            bullet.rot += 0.1;
        }
    }
}
