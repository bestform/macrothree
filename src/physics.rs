use macroquad::prelude::*;

use crate::{ENEMY_SIZE, Particle};
use crate::audio::Audio;
use crate::audio::SFX::EXPLOSION;
use crate::game::Game;
use crate::structs::{PointsToAdd};

impl Game {

    pub fn handle_collisions(&mut self, audio: &mut Audio) {
        for bullet in self.bullets.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if enemy.pos.abs_diff_eq(bullet.pos, ENEMY_SIZE) {
                    bullet.alive = false;
                    enemy.hitpoints -= 1;

                    // todo: this logic should not be in here. But well..
                    if enemy.hitpoints == 0 {
                        self.points_to_add.push(PointsToAdd{ amount: 100, pos: enemy.pos });
                        audio.play_sfx(EXPLOSION);
                    } else {
                        self.points_to_add.push(PointsToAdd{ amount: 60, pos: enemy.pos });
                    }

                    for _ in 0..10 {
                        let factor = rand::gen_range(0.5, 1.0);
                        let color = Color::new(factor, 0.2, 0.2, factor);

                        self.particles.push(Particle{
                            pos: bullet.pos,
                            vel: Vec2::new(rand::gen_range(-2.5, 2.5), rand::gen_range(-2.5, 2.5)),
                            size: rand::gen_range(1., 2.5),
                            color,
                            created_at: get_time()
                        })
                    }
                }
            }
        }
    }
}
