use crate::structs::{Enemy, PointsToAdd};
use crate::{Bullet, ENEMY_SIZE, Particle};
use macroquad::prelude::*;

pub fn handle_bullets_collision(
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    particles: &mut Vec<Particle>,
    points_to_add: &mut Vec<PointsToAdd>
) {
    for bullet in bullets.iter_mut() {
        for enemy in enemies.iter_mut() {
            if enemy.pos.abs_diff_eq(bullet.pos, ENEMY_SIZE) {
                bullet.alive = false;
                enemy.hitpoints -= 1;

                // todo: this logic should not be in here. But well..
                if enemy.hitpoints == 0 {
                    points_to_add.push(PointsToAdd{ amount: 100, pos: enemy.pos });
                } else {
                    points_to_add.push(PointsToAdd{ amount: 60, pos: enemy.pos });
                }

                for _ in 0..10 {
                    let factor = rand::gen_range(0.5, 1.0);
                    let color = Color::new(factor, 0.2, 0.2, factor);

                    particles.push(Particle{
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