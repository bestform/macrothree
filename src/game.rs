use macroquad::prelude::*;

use crate::{BOTTOM_MARGIN, PLAYER_SIZE, PLAYER_SPEED, TOP_MARGIN};
use crate::input::{handle_player_input, handle_shortcuts};
use crate::math::{clampx, clampy};
use crate::structs::{Bullet, Enemy, FloatingMessage, Particle, Player, PlayerMovementState, PointsToAdd, Star};
use crate::state::GameState;

pub struct Game {
    pub(crate) player: Player,
    pub(crate) bullets: Vec<Bullet>,
    pub(crate) enemy_bullets: Vec<Bullet>,
    pub(crate) stars: Vec<Star>,
    pub(crate) particles: Vec<Particle>,
    pub(crate) enemies: Vec<Enemy>,
    pub(crate) points_to_add: Vec<PointsToAdd>,
    pub(crate) messages: Vec<FloatingMessage>,
    pub(crate) last_particle_t: f64,
    pub(crate) last_enemy_t: f64,
    pub(crate) total_points: i32,

    pub(crate) ship_tx: Texture2D,
    pub(crate) bullet_tx: Texture2D,
    pub(crate) enemy_ship_textures: Vec<Texture2D>,
    pub(crate) enemy_bullet_01_tx: Texture2D,

    pub(crate) font: Font,
}

impl Game {
    pub async fn new() -> Self {
        let player = Player {
            pos: Vec2::new(screen_width() / 2., screen_height() - PLAYER_SIZE - BOTTOM_MARGIN),
            vel: Vec2::new(0., 0.),
            last_shot: get_time(),
        };


        let bullets: Vec<Bullet> = Vec::new();
        let enemy_bullets: Vec<Bullet> = Vec::new();
        let stars: Vec<Star> = Vec::new();
        let particles: Vec<Particle> = Vec::new();
        let enemies: Vec<Enemy> = Vec::new();
        let points_to_add: Vec<PointsToAdd> = Vec::new();
        let messages: Vec<FloatingMessage> = Vec::new();
        let last_particle_t: f64 = get_time();
        let last_enemy_t: f64 = get_time();
        let total_points: i32 = 0;

        let ship_tx = load_texture("Resources/player_ship.png").await;
        let bullet_tx = load_texture("Resources/bullet.png").await;
        let enemy_ship_01_tx = load_texture("Resources/enemy_ship_01.png").await;
        let enemy_ship_02_tx = load_texture("Resources/enemy_ship_02.png").await;
        let enemy_ship_03_tx = load_texture("Resources/enemy_ship_03.png").await;
        let enemy_ship_04_tx = load_texture("Resources/enemy_ship_04.png").await;
        let enemy_bullet_01_tx = load_texture("Resources/enemy_bullet_01.png").await;

        let enemy_ship_textures = vec![enemy_ship_01_tx, enemy_ship_02_tx, enemy_ship_03_tx, enemy_ship_04_tx];

        let font = load_ttf_font("Resources/LASER.ttf").await;

        Self {
            player,
            bullets,
            enemy_bullets,
            stars,
            particles,
            enemies,
            points_to_add,
            messages,
            last_particle_t,
            last_enemy_t,
            total_points,

            ship_tx,
            bullet_tx,
            enemy_ship_textures,
            enemy_bullet_01_tx,

            font,
        }
    }

    pub fn handle_player_move_for_frame(&mut self) -> GameState {
        self.handle_player_movement();
        return handle_shortcuts();
    }

    pub fn handle_points(&mut self, frame_t: f64) {
        for p in self.points_to_add.iter() {
            self.messages.push(FloatingMessage {
                message: p.amount.to_string(),
                shown_at: frame_t,
                pos: p.pos,
                scale: p.amount as f32 / 100.
            });
            self.total_points += p.amount;
        }

        self.points_to_add.clear();
    }

    fn handle_player_movement(&mut self) {
        let player_movement_states = handle_player_input();

        for state in player_movement_states {
            match state {
                PlayerMovementState::IDLE => { self.player.vel = Vec2::new(0., 0.); }
                PlayerMovementState::LEFT => { self.player.vel.set_x(-1. * PLAYER_SPEED); }
                PlayerMovementState::RIGHT => { self.player.vel.set_x(PLAYER_SPEED); }
                PlayerMovementState::UP => { self.player.vel.set_y(-1. * PLAYER_SPEED); }
                PlayerMovementState::DOWN => { self.player.vel.set_y(PLAYER_SPEED); }
            }
        }

        if self.player.vel.length() == 0. {
            return;
        }

        self.player.vel = self.player.vel.normalize() * PLAYER_SPEED;
        self.player.pos += self.player.vel;
        self.player.pos = clampx(self.player.pos, 0., screen_width());
        self.player.pos = clampy(self.player.pos, TOP_MARGIN, screen_height() - PLAYER_SIZE - BOTTOM_MARGIN);
    }

}
