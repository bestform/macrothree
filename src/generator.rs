use crate::{Star, STAR_DENSITY, Particle, Player, PARTICLE_DENSITY, PLAYER_SIZE, Bullet, BULLET_SPEED};
use macroquad::prelude::*;

pub fn create_stars(stars: &mut Vec<Star>, frame_t: f64) {
    //println!("{}", frame_t);
    if frame_t % 1. < STAR_DENSITY {
        stars.push(Star{
            pos: Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(-20., 0.)),
            vel: Vec2::new(0., rand::gen_range(2., 2.5)),
            size: rand::gen_range(1., 2.),
            brightness: rand::gen_range(0.1, 1.)
        })
    }
}

pub fn create_particles(player: Player, particles: &mut Vec<Particle>, frame_t: f64, last_particle_t: &mut f64) {
    if frame_t - *last_particle_t > PARTICLE_DENSITY {
        particles.push(Particle {
            pos: player.pos + Vec2::new(0.0, PLAYER_SIZE),
            vel: Vec2::new(rand::gen_range(-0.8, 0.8), rand::gen_range(3., 4.5)),
            size: rand::gen_range(1.0, 2.0),
            color: YELLOW,
            created_at: frame_t
        });
        *last_particle_t = frame_t;
    }
}

pub fn create_boolets(player: &mut Player, bullets: &mut Vec<Bullet>, frame_t: f64) {
    if is_key_down(KeyCode::Space) && frame_t - player.last_shot > 0.1 {
        bullets.push(Bullet{
            pos: player.pos + Vec2::new(0., 5.),
            vel: Vec2::new(0., BULLET_SPEED),
            alive: true
        });
        player.last_shot = frame_t;
    }
}