use macroquad::prelude::*;

use crate::{ENEMY_SIZE, Particle, PLAYER_SIZE, PLAYER_COLLISION};
use crate::audio::Audio;
use crate::audio::SFX::EXPLOSION;
use crate::game::Game;
use crate::structs::{PointsToAdd};
use crate::state::GameState;

impl Game {

    pub fn handle_collisions(&mut self, audio: &mut Audio, current_game_state: GameState) -> GameState {
        let mut new_game_state = current_game_state;
        // bullets on enemies
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

                    let particles = create_hit_particles(bullet.pos);
                    for p in particles.iter() {
                        self.particles.push(p.clone());
                    }
                }
            }
        }
        // bullets on player
        for bullet in self.enemy_bullets.iter_mut() {
            if !self.player.pos.abs_diff_eq(bullet.pos, PLAYER_COLLISION) {
                continue;
            }
            bullet.alive = false;
            self.player.health -= 10;

            let particles = create_hit_particles(bullet.pos);
            for p in particles.iter() {
                self.particles.push(p.clone());
            }

            if self.player.health <= 0 {
                new_game_state = GameState::GameOver;
            }
        }

        new_game_state
    }

}

fn create_hit_particles(pos: Vec2) -> Vec<Particle> {
    let mut particles = Vec::new();
    for _ in 0..10 {
        let factor = rand::gen_range(0.5, 1.0);
        let color = Color::new(factor, 0.2, 0.2, factor);

        particles.push(Particle{
            pos,
            vel: Vec2::new(rand::gen_range(-2.5, 2.5), rand::gen_range(-2.5, 2.5)),
            size: rand::gen_range(1., 2.5),
            color,
            created_at: get_time()
        })
    }

    return particles;
}
