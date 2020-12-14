mod debug;
mod renderer;
mod math;
mod lifetime;
mod generator;
mod movement;
mod input;
mod structs;
mod physics;
mod audio;

use macroquad::prelude::*;
use crate::renderer::*;
use crate::lifetime::*;
use crate::generator::*;
use crate::movement::*;
use crate::input::*;
use crate::structs::*;
use crate::physics::*;
use crate::audio::{Audio};

const DEBUG: bool = true;

const PLAYER_SPEED: f32 = 7.;
const BOTTOM_MARGIN: f32 = 20.;
const TOP_MARGIN: f32 = 200.;
const SIDE_MARGIN: f32 = 200.;
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
const ENABLE_PARALLAX: bool = false;
const MESSAGE_LIFETIME: f64 = 1.;
const POINTS_COLOR: Color = Color::new(0.6, 0.6, 0.9, 1.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroThree".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut audio = Audio::new();
    audio.play_background_music();

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
    let mut points_to_add: Vec<PointsToAdd> = Vec::new();
    let mut messages: Vec<FloatingMessage> = Vec::new();
    let mut last_particle_t: f64 = get_time();
    let mut last_enemy_t: f64 = get_time();
    let mut total_points: i32 = 0;

    let font = load_ttf_font("Resources/LASER.ttf").await;

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

        create_bullets(&mut player, &mut bullets, &mut audio, frame_t);
        create_enemies(&mut enemies, frame_t, &mut last_enemy_t);
        create_engine_particles(player, &mut particles, frame_t, &mut last_particle_t);
        create_enemy_bullets(&mut enemies, &mut enemy_bullets, frame_t);
        create_stars(&mut stars, frame_t);

        handle_enemies_move(&mut enemies);
        handle_bullets_move(&mut bullets, &mut enemy_bullets);
        handle_particles_move(&mut particles);
        handle_messages_move(&mut messages);
        handle_stars_move(&mut stars, handle_player_input());

        handle_bullet_lifetime(&mut bullets, &mut enemy_bullets);
        handle_particles_lifetime(&mut particles, frame_t);
        handle_enemies_lifetime(&mut enemies);
        handle_stars_lifetime(&mut stars);
        handle_messages_lifetime(&mut messages, frame_t);

        handle_bullets_collision(&mut enemies, &mut bullets, &mut particles, &mut points_to_add, &mut audio);

        handle_points_to_add(&mut points_to_add, &mut messages, &mut total_points, frame_t);

        // DRAW
        clear_background(BLACK);
        draw_stars(stars.clone());
        draw_particles(particles.clone());
        draw_messages(messages.clone(), font);
        draw_player(player, ship_tx);
        draw_enemies(enemies.clone(), enemy_ship_textures.clone());
        draw_bullets(bullets.clone(), enemy_bullets.clone(), bullet_tx, enemy_bullet_01_tx);
        draw_total_points(total_points, font);
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

fn handle_points_to_add(points_to_add: &mut Vec<PointsToAdd>, messages: &mut Vec<FloatingMessage>, total_points: &mut i32, frame_t: f64) {
    for p in points_to_add.iter() {
        messages.push(FloatingMessage {
            message: p.amount.to_string(),
            shown_at: frame_t,
            pos: p.pos,
            scale: p.amount as f32 / 100.
        });
        *total_points += p.amount;
    }

    points_to_add.clear();
}
