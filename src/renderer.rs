use macroquad::prelude::*;
use crate::{Particle, Star, Bullet, BULLET_SIZE, Player, PLAYER_SIZE};

pub fn draw_particles(particles: Vec<Particle>) {
    for particle in particles {
        draw_circle(
            particle.pos.x(),
            particle.pos.y(),
            particle.size,
            particle.color
        );
    }
}

pub fn draw_stars(stars: Vec<Star>) {
    for star in stars{
        let color = Color::new(1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness);
        draw_circle(
            star.pos.x(),
            star.pos.y(),
            star.size,
            color
        );
    }

}

pub fn draw_bullets(bullets: Vec<Bullet>, bullet_tx: Texture2D) {
    for bullet in bullets {
        draw_texture_ex(
            bullet_tx,
            bullet.pos.x() - BULLET_SIZE / 2.,
            bullet.pos.y(),
            WHITE,
            DrawTextureParams{
                dest_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                source: None,
                rotation: 0.0,
                pivot: None
            }
        )
    }
}

pub fn draw_player(player: Player, ship_tx: Texture2D) {
    draw_texture_ex(
        ship_tx,
        player.pos.x() - PLAYER_SIZE / 2.,
        player.pos.y(),
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            source: None,
            rotation: 0.0,
            pivot: None
        }
    );
}