use macroquad::prelude::*;
use crate::{Particle, Star, Bullet, BULLET_SIZE, Player, PLAYER_SIZE, ENEMY_SIZE, ENEMY_BULLET_SIZE, POINTS_COLOR};
use crate::structs::{Enemy, FloatingMessage};

pub fn draw_particles(particles: Vec<Particle>) {
    for particle in particles {
        draw_circle(
            particle.pos.x(),
            particle.pos.y(),
            particle.size,
            particle.color,
        );
    }
}

pub fn draw_messages(messages: Vec<FloatingMessage>, font: Font) {
    let font_size = 30;
    for m in messages {
        let text_size = measure_text(&m.message, Some(font), font_size as _, m.scale);
        draw_text_ex(
            &m.message,
            m.pos.x() - text_size.0 / 2.,
            m.pos.y(),
            TextParams {
                font,
                font_size,
                font_scale: m.scale,
                color: POINTS_COLOR,
            },
        );
    }
}

// todo: brighter stars should move with the player position (parallax)
pub fn draw_stars(stars: Vec<Star>) {
    for star in stars {
        let color = Color::new(1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness);
        draw_circle(
            star.pos.x(),
            star.pos.y(),
            star.size,
            color,
        );
    }
}

pub fn draw_enemies(enemies: Vec<Enemy>, textures: Vec<Texture2D>) {
    for enemy in enemies {
        let texture = textures.get(enemy.tex_idx).unwrap();
        draw_texture_ex(
            *texture,
            enemy.pos.x() - ENEMY_SIZE / 2.,
            enemy.pos.y(),
            WHITE,
            DrawTextureParams{
                dest_size: Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                source: None,
                rotation: 0.0,
                pivot: None
            }
        );
        let health_percent:f32 = enemy.hitpoints as f32 / 5.0;
        let bar_width = 70.;
        let bar_height = 10.;
        let bar_border = 2.;
        draw_rectangle(
            enemy.pos.x() - bar_width / 2.,
            enemy.pos.y() - ENEMY_SIZE / 2.,
            bar_width,
            bar_height,
            DARKGRAY
        );
        draw_rectangle(
            enemy.pos.x() - bar_width / 2. + bar_border,
            enemy.pos.y() - ENEMY_SIZE / 2. + bar_border,
            bar_width * health_percent - bar_border * 2.,
            bar_height - bar_border * 2.,
            RED
        );
    }
}

pub fn draw_bullets(bullets: Vec<Bullet>, enemy_bullets: Vec<Bullet>, bullet_tx: Texture2D, enemy_bullet_tx: Texture2D) {
    for bullet in bullets {
        draw_texture_ex(
            bullet_tx,
            bullet.pos.x() - BULLET_SIZE / 2.,
            bullet.pos.y(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                source: None,
                rotation: 0.0,
                pivot: None,
            },
        )
    }
    for bullet in enemy_bullets{
        draw_texture_ex(
            enemy_bullet_tx,
            bullet.pos.x() - ENEMY_BULLET_SIZE / 2.,
            bullet.pos.y(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(ENEMY_BULLET_SIZE, ENEMY_BULLET_SIZE)),
                source: None,
                rotation: bullet.rot,
                pivot: Some(bullet.pos + Vec2::new(0., ENEMY_BULLET_SIZE / 2.)),
            },
        );
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
            pivot: None,
        },
    );
}

pub fn draw_total_points(points: i32, font: Font) {
    let font_size = 30;
    let text = std::fmt::format(format_args!("POINTS: {}", points));
    let text_size = measure_text(&text, Some(font), font_size as _, 1.0);
    draw_text_ex(
        &text,
        20.,
        screen_height() - text_size.1 - 20.,
        TextParams {
            font,
            font_size,
            font_scale: 1.0,
            color: POINTS_COLOR,
        },
    );
}