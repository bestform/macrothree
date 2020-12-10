mod debug;
mod renderer;
mod math;
mod lifetime;
mod generator;
mod movement;

use macroquad::prelude::*;
use std::process::exit;
use crate::renderer::{draw_stars, draw_particles, draw_player, draw_bullets};
use crate::lifetime::{handle_bullet_lifetime, handle_particles_lifetime, handle_stars_lifetime};
use crate::generator::{create_particles, create_stars, create_boolets};
use crate::movement::{handle_player_movement, handle_bullet_move, handle_particles_move, handle_stars_move};

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
const BULLET_SIZE: f32 = 7.;
const STAR_DENSITY: f64 = 0.5;
const PARTICLE_DENSITY: f64 = 0.02;
const PARTICLE_LIFETIME: f64 = 0.9;

#[macroquad::main("MacroThree")]
async fn main() {
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2., screen_height() - PLAYER_SIZE - BOTTOM_MARGIN),
        vel: Vec2::new(0., 0.),
        last_shot: get_time(),
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut stars: Vec<Star> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut last_particle_t: f64 = get_time();

    let ship_tx = load_texture("Resources/player_ship.png").await;
    let bullet_tx = load_texture("Resources/bullet.png").await;

    loop {
        let frame_t = get_time();

        // MOVE PLAYER
        handle_player_movement(&mut player);
        handle_shortcuts();

        create_boolets(&mut player, &mut bullets, frame_t);
        handle_bullet_move(&mut bullets);
        handle_bullet_lifetime(&mut bullets);

        create_particles(player, &mut particles, frame_t, &mut last_particle_t);
        handle_particles_move(&mut particles);
        handle_particles_lifetime(&mut particles, frame_t);

        create_stars(&mut stars, frame_t);
        handle_stars_move(&mut stars);
        handle_stars_lifetime(&mut stars);


        // DRAW
        clear_background(BLACK);
        draw_stars(stars.clone());
        draw_particles(particles.clone());
        draw_player(player, ship_tx);
        draw_bullets(bullets.clone(), bullet_tx);
        if DEBUG {
            debug::draw_debug(stars.clone(), bullets.clone(), particles.clone());
        }
        next_frame().await
    }
}


fn handle_shortcuts() {
    if is_key_down(KeyCode::Q) {
        exit(0);
    }
}


















