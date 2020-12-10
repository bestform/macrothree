mod debug;
mod renderer;
mod math;
mod lifetime;
mod generator;
mod movement;
mod input;
mod structs;
mod physics;

use macroquad::prelude::*;
use crate::renderer::{draw_stars, draw_particles, draw_player, draw_bullets, draw_enemies};
use crate::lifetime::{handle_bullet_lifetime, handle_particles_lifetime, handle_stars_lifetime, handle_enemies_lifetime};
use crate::generator::{create_engine_particles, create_stars, create_bullets, create_enemies, create_enemy_bullets};
use crate::movement::{handle_player_movement, handle_bullets_move, handle_particles_move, handle_stars_move, handle_enemies_move};
use crate::input::{handle_shortcuts, handle_player_input};
use crate::structs::Enemy;
use crate::physics::handle_bullets_collision;

#[derive(Clone, Copy)]
pub struct Star {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    brightness: f32,
}

#[derive(Clone)]
pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    color: Color,
    created_at: f64,
}

#[derive(Clone)]
pub struct Bullet {
    pos: Vec2,
    vel: Vec2,
    alive: bool,
    rot: f32,
}

pub enum PlayerMovementState {
    IDLE,
    LEFT,
    RIGHT,
    UP,
    DOWN
}

#[derive(Clone, Copy)]
pub struct Player {
    pos: Vec2,
    vel: Vec2,
    last_shot: f64,
}

const DEBUG: bool = true;

const PLAYER_SPEED: f32 = 4.;
const BOTTOM_MARGIN: f32 = 20.;
const TOP_MARGIN: f32 = 200.;
const PLAYER_SIZE: f32 = 100.;
const BULLET_SPEED: f32 = -9.;
const ENEMY_BULLET_SPEED: f32 = 6.;
const BULLET_SIZE: f32 = 7.;
const ENEMY_BULLET_SIZE: f32 = 22.;
const STAR_DENSITY: f64 = 0.2;
const PARTICLE_DENSITY: f64 = 0.02;
const PARTICLE_LIFETIME: f64 = 0.9;
const ENEMY_FREQ: f64 = 2.;
const ENEMY_SIZE: f32 = 50.;

#[macroquad::main("MacroThree")]
async fn main() {
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2., screen_height() - PLAYER_SIZE - BOTTOM_MARGIN),
        vel: Vec2::new(0., 0.),
        last_shot: get_time(),
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemy_bullets: Vec<Bullet> = Vec::new();
    let mut stars: Vec<Star> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut last_particle_t: f64 = get_time();
    let mut last_enemy_t: f64 = get_time();

    let ship_tx = load_texture("Resources/player_ship.png").await;
    let bullet_tx = load_texture("Resources/bullet.png").await;
    let enemy_ship_01_tx = load_texture("Resources/enemy_ship_01.png").await;
    let enemy_ship_02_tx = load_texture("Resources/enemy_ship_02.png").await;
    let enemy_ship_03_tx = load_texture("Resources/enemy_ship_03.png").await;
    let enemy_ship_04_tx = load_texture("Resources/enemy_ship_04.png").await;
    let enemy_bullet_01_tx = load_texture("Resources/enemy_bullet_01.png").await;

    let enemy_ship_textures = vec![enemy_ship_01_tx, enemy_ship_02_tx, enemy_ship_03_tx, enemy_ship_04_tx];

    loop {
        let frame_t = get_time();

        // MOVE PLAYER
        handle_player_movement(&mut player, handle_player_input());
        handle_shortcuts();

        create_bullets(&mut player, &mut bullets, frame_t);
        create_enemies(&mut enemies, frame_t, &mut last_enemy_t);
        create_engine_particles(player, &mut particles, frame_t, &mut last_particle_t);
        create_enemy_bullets(&mut enemies, &mut enemy_bullets, frame_t);
        create_stars(&mut stars, frame_t);

        handle_enemies_move(&mut enemies);
        handle_bullets_move(&mut bullets, &mut enemy_bullets);
        handle_particles_move(&mut particles);
        handle_stars_move(&mut stars, handle_player_input());

        handle_bullet_lifetime(&mut bullets, &mut enemy_bullets);
        handle_particles_lifetime(&mut particles, frame_t);
        handle_enemies_lifetime(&mut enemies);
        handle_stars_lifetime(&mut stars);

        handle_bullets_collision(&mut enemies, &mut bullets, &mut particles);



        // DRAW
        clear_background(BLACK);
        draw_stars(stars.clone());
        draw_particles(particles.clone());
        draw_player(player, ship_tx);
        draw_enemies(enemies.clone(), enemy_ship_textures.clone());
        draw_bullets(bullets.clone(), enemy_bullets.clone(), bullet_tx, enemy_bullet_01_tx);
        if DEBUG {
            debug::draw_debug(
                stars.clone(),
                bullets.clone(),
                enemy_bullets.clone(),
                particles.clone(),
                enemies.clone()
            );
        }
        next_frame().await
    }
}





















