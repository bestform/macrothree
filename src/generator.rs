use crate::{Star, STAR_DENSITY, Particle, Player, PARTICLE_DENSITY, PLAYER_SIZE, Bullet, BULLET_SPEED, ENEMY_FREQ, ENEMY_SIZE, ENEMY_BULLET_SPEED};
use macroquad::prelude::*;
use crate::structs::Enemy;

pub fn create_stars(stars: &mut Vec<Star>, frame_t: f64) {
    //println!("{}", frame_t);
    if frame_t % 1. < STAR_DENSITY {
        stars.push(Star {
            pos: Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(-20., 0.)),
            vel: Vec2::new(0., rand::gen_range(2., 2.5)),
            size: rand::gen_range(1., 2.),
            brightness: rand::gen_range(0.1, 1.),
        })
    }
}

pub fn create_engine_particles(player: Player, particles: &mut Vec<Particle>, frame_t: f64, last_particle_t: &mut f64) {
    if frame_t - *last_particle_t > PARTICLE_DENSITY {
        let factor = rand::gen_range(0.3, 0.7);
        let color = Color::new(
            1.0 * factor,
            0.8,
            0.8,
            factor + 0.3
        );
        particles.push(Particle {
            pos: player.pos + Vec2::new(0.0, PLAYER_SIZE),
            vel: Vec2::new(rand::gen_range(-0.8, 0.8), rand::gen_range(3., 4.5)),
            size: rand::gen_range(1.0, 2.0),
            color,
            created_at: frame_t,
        });
        *last_particle_t = frame_t;
    }
}

pub fn create_bullets(player: &mut Player, bullets: &mut Vec<Bullet>, frame_t: f64) {
    if is_key_down(KeyCode::Space) && frame_t - player.last_shot > 0.1 {
        bullets.push(Bullet {
            pos: player.pos + Vec2::new(0., 5.),
            vel: Vec2::new(0., BULLET_SPEED),
            alive: true,
            rot: 0.
        });
        player.last_shot = frame_t;
    }
}

pub fn create_enemy_bullets(enemies: &mut Vec<Enemy>, enemy_bullets: &mut Vec<Bullet>, frame_t: f64) {
    for enemy in enemies.iter_mut() {
        if frame_t - enemy.last_shot > enemy.shot_freq{
            enemy_bullets.push(Bullet {
                pos: enemy.pos + Vec2::new(0., ENEMY_SIZE),
                vel: Vec2::new(0., ENEMY_BULLET_SPEED),
                alive: true,
                rot: 0.
            });
            enemy.last_shot = frame_t;
        }
    }
}

pub fn create_enemies(enemies: &mut Vec<Enemy>, frame_t: f64, last_enemy_t: &mut f64) {
    if frame_t - *last_enemy_t > ENEMY_FREQ {
        enemies.push(Enemy {
            pos: Vec2::new(rand::gen_range(0., screen_width()), -ENEMY_SIZE),
            vel: Vec2::new(0., rand::gen_range(2., 4.)),
            hitpoints: 5,
            tex_idx: rand::gen_range::<usize>(0, 4),
            last_shot: frame_t,
            shot_freq: rand::gen_range(1., 4.)
        });

        *last_enemy_t = frame_t;
    }

}