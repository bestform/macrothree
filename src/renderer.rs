use macroquad::prelude::*;
use crate::{BULLET_SIZE, PLAYER_SIZE, ENEMY_SIZE, ENEMY_BULLET_SIZE, POINTS_COLOR, DEBUG, debug};
use crate::game::Game;


impl Game {

    pub fn draw(&self) {

        clear_background(BLACK);
        self.draw_stars();
        self.draw_particles();
        self.draw_messages();
        self.draw_player();
        self.draw_enemies();
        self.draw_bullets();
        self.draw_total_points();
        if DEBUG {
            debug::draw_debug(
                self.stars.clone(),
                self.bullets.clone(),
                self.enemy_bullets.clone(),
                self.particles.clone(),
                self.enemies.clone()
            );
        }
    }

    fn draw_particles(&self) {
        for particle in self.particles.iter() {
            draw_circle(
                particle.pos.x(),
                particle.pos.y(),
                particle.size,
                particle.color,
            );
        }
    }

    fn draw_messages(&self) {
        let font_size = 30;
        for m in self.messages.iter() {
            let text_size = measure_text(&m.message, Some(self.font), font_size as _, m.scale);
            let font = self.font;
            draw_text_ex(
                &m.message,
                m.pos.x - text_size.width / 2.,
                m.pos.y,
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
    fn draw_stars(&self) {
        for star in self.stars.iter() {
            let color = Color::new(1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness, 1.00 * star.brightness);
            draw_circle(
                star.pos.x(),
                star.pos.y(),
                star.size,
                color,
            );
        }
    }

    fn draw_enemies(&self) {
        for enemy in self.enemies.iter() {
            let texture = self.enemy_ship_textures.get(enemy.tex_idx).unwrap();
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

    fn draw_bullets(&self) {
        for bullet in self.bullets.iter() {
            draw_texture_ex(
                self.bullet_tx,
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
        for bullet in self.enemy_bullets.iter() {
            draw_texture_ex(
                self.enemy_bullet_01_tx,
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

    fn draw_player(&self) {
        draw_texture_ex(
            self.ship_tx,
            self.player.pos.x() - PLAYER_SIZE / 2.,
            self.player.pos.y(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                source: None,
                rotation: 0.0,
                pivot: None,
            },
        );
    }

    fn draw_total_points(&self) {
        let font_size = 30;
        let text = std::fmt::format(format_args!("POINTS: {}", self.total_points));
        let text_size = measure_text(&text, Some(self.font), font_size as _, 1.0);
        let font = self.font;
        draw_text_ex(
            &text,
            20.,
            screen_height() - text_size.height - 20.,
            TextParams {
                font,
                font_size,
                font_scale: 1.0,
                color: POINTS_COLOR,
            },
        );
    }
}
