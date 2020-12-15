use macroquad::prelude::*;

use crate::{Bullet, BULLET_SPEED, ENEMY_BULLET_SPEED, ENEMY_FREQ, ENEMY_SIZE, Particle, PARTICLE_DENSITY, PLAYER_SIZE, SIDE_MARGIN, Star, STAR_DENSITY};
use crate::audio::Audio;
use crate::audio::SFX::SHOOT;
use crate::game::Game;
use crate::structs::Enemy;

impl Game {
    pub fn handle_create_for_frame(&mut self, audio: &mut Audio, frame_t: f64) {
        self.create_bullets(audio, frame_t);
        self.create_enemies(frame_t);
        self.create_engine_particles(frame_t);
        self.create_enemy_bullets(frame_t);
        self.create_stars(frame_t);
    }

    fn create_stars(&mut self, frame_t: f64) {
        if frame_t % 1. < STAR_DENSITY {
            let distance = rand::gen_range(0.1, 1.);
            self.stars.push(Star {
                pos: Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(-20., 0.)),
                vel: Vec2::new(0., 2.5 * distance),
                size: rand::gen_range(1., 2.),
                brightness: distance,
            })
        }
    }

    fn create_enemy_bullets(&mut self, frame_t: f64) {
        for enemy in self.enemies.iter_mut() {
            if frame_t - enemy.last_shot > enemy.shot_freq{
                self.enemy_bullets.push(Bullet {
                    pos: enemy.pos + Vec2::new(0., ENEMY_SIZE),
                    vel: Vec2::new(0., ENEMY_BULLET_SPEED),
                    alive: true,
                    rot: 0.
                });
                enemy.last_shot = frame_t;
            }
        }
    }

    fn create_engine_particles(&mut self, frame_t: f64) {
        if frame_t - self.last_particle_t > PARTICLE_DENSITY {
            let factor = rand::gen_range(0.3, 0.7);
            let color = Color::new(
                1.0 * factor,
                0.8,
                0.8,
                factor + 0.3
            );
            self.particles.push(Particle {
                pos: self.player.pos + Vec2::new(0.0, PLAYER_SIZE),
                vel: Vec2::new(rand::gen_range(-0.8, 0.8), rand::gen_range(3., 4.5)),
                size: rand::gen_range(1.0, 2.0),
                color,
                created_at: frame_t,
            });
            self.last_particle_t = frame_t;
        }
    }


    fn create_bullets(&mut self, audio: &mut Audio, frame_t: f64) {
        if is_key_down(KeyCode::Space) && frame_t - self.player.last_shot > 0.1 {
            self.bullets.push(Bullet {
                pos: self.player.pos + Vec2::new(0., 5.),
                vel: Vec2::new(0., BULLET_SPEED),
                alive: true,
                rot: 0.
            });
            // todo: this is NOT the place to trigger this. This is just here to test things.
            audio.play_sfx(SHOOT);
            self.player.last_shot = frame_t;
        }
    }

    fn create_enemies(&mut self, frame_t: f64) {
        if frame_t - self.last_enemy_t > ENEMY_FREQ {
            self.enemies.push(Enemy {
                pos: Vec2::new(rand::gen_range(SIDE_MARGIN, screen_width() - SIDE_MARGIN), -ENEMY_SIZE),
                vel: Vec2::new(0., rand::gen_range(2., 4.)),
                hitpoints: 5,
                tex_idx: rand::gen_range::<usize>(0, 4),
                last_shot: frame_t,
                shot_freq: rand::gen_range(1., 4.)
            });

            self.last_enemy_t = frame_t;
        }
    }
}
