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
mod game;
mod state;

use macroquad::prelude::*;
use crate::structs::*;
use crate::audio::{Audio};
use crate::game::Game;
use crate::state::{GameState, RunState};
use std::process::exit;

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

    let mut state = GameState::Playing;

    let mut game = Game::new().await;

    loop {
        let frame_t = get_time();

        match state {
            GameState::Menu => {}
            GameState::Playing => {
                state = game.run(frame_t, &mut audio);
            }
            GameState::GameOver => {
                exit(0);
            }
        }





        next_frame().await
    }
}
