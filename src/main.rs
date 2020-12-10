mod debug;

use macroquad::prelude::*;
use std::process::exit;

enum PebbleColor {
    Red,
    Blue,
    Green,
}

struct Pepple {
    pos: Vec2,
    color: PebbleColor
}

#[derive(Clone, Copy)]
pub struct Star {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    brightness: f32,
}

#[derive(Clone)]
pub struct Bullet {
    pos: Vec2,
    vel: Vec2,
    alive: bool,
}

#[derive(Clone, Copy)]
struct Player {
    pos: Vec2,
    vel: Vec2,
    last_shot: f64
}

const DEBUG:bool = true;

const PLAYER_SPEED: f32 = 4.;
const BOTTOM_MARGIN: f32 = 20.;
const TOP_MARGIN: f32 = 200.;
const PLAYER_SIZE: f32 = 100.;
const BULLET_SPEED: f32 = -9.;
const BULLET_SIZE: f32 = 7.;
const STAR_DENSITY: f64 = 0.5;
#[macroquad::main("MacroThree")]
async fn main() {

    let mut player = Player {
        pos: Vec2::new(screen_width() / 2., screen_height() - PLAYER_SIZE - BOTTOM_MARGIN),
        vel: Vec2::new(0., 0.),
        last_shot: get_time()
    };

    let mut bullets:Vec<Bullet> = Vec::new();
    let mut stars:Vec<Star> = Vec::new();

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
        create_stars(&mut stars, frame_t);
        handle_stars_move(&mut stars);
        handle_stars_lifetime(&mut stars);


        // DRAW
        clear_background(BLACK);
        draw_stars(stars.clone());
        draw_player(player, ship_tx);
        draw_bullets(bullets.clone(), bullet_tx);
        if DEBUG {
            debug::draw_debug(stars.clone(), bullets.clone());
        }
        next_frame().await
    }
}

fn handle_shortcuts() {
    if is_key_down(KeyCode::Q) {
        exit(0);
    }
}

fn handle_stars_lifetime(stars: &mut Vec<Star>) {
    stars.retain(|s| s.pos.y() < screen_height());
}

fn draw_stars(stars: Vec<Star>) {
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

fn handle_stars_move(stars: &mut Vec<Star>) {
    for star in stars {
        star.pos += star.vel;
    }
}

fn create_stars(stars: &mut Vec<Star>, frame_t: f64) {
    //println!("{}", frame_t);
    if frame_t % 1. < STAR_DENSITY {
        stars.push(Star{
            pos: Vec2::new(rand::gen_range(0., screen_width()), rand::gen_range(-20., 0.)),
            vel: Vec2::new(0., rand::gen_range(2., 2.5)),
            size: rand::gen_range(1., 2.),
            brightness: rand::gen_range(0.1, 1.)
        })
    }
}

fn handle_bullet_lifetime(bullets: &mut Vec<Bullet>) {
    bullets.retain(|bullet| bullet.pos.y() > 0.);
}

fn handle_bullet_move(bullets: &mut Vec<Bullet>) {
    for bullet in bullets {
        bullet.pos += bullet.vel;
    }
}

fn draw_bullets(bullets: Vec<Bullet>, bullet_tx: Texture2D) {
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

fn create_boolets(player: &mut Player, bullets: &mut Vec<Bullet>, frame_t: f64) {
    if is_key_down(KeyCode::Space) && frame_t - player.last_shot > 0.1 {
        bullets.push(Bullet{
            pos: player.pos + Vec2::new(0., 5.),
            vel: Vec2::new(0., BULLET_SPEED),
            alive: true
        });
        player.last_shot = frame_t;
    }
}

fn handle_player_movement(player: &mut Player) {
    player.vel = Vec2::new(0., 0.);

    if !(is_key_down(KeyCode::A) && is_key_down(KeyCode::D)) {
        if is_key_down(KeyCode::A) {
            player.vel.set_x(-1. * PLAYER_SPEED);
        }
        if is_key_down(KeyCode::D) {
            player.vel.set_x(PLAYER_SPEED);
        }
    }
    if !(is_key_down(KeyCode::W) && is_key_down(KeyCode::S)) {
        if is_key_down(KeyCode::W) {
            player.vel.set_y(-1. * PLAYER_SPEED);
        }
        if is_key_down(KeyCode::S) {
            player.vel.set_y(PLAYER_SPEED);
        }
    }

    if player.vel.length() == 0. {
        return;
    }

    player.vel = player.vel.normalize() * PLAYER_SPEED;
    player.pos += player.vel;
    player.pos = clampx(player.pos, 0., screen_width());
    player.pos = clampy(player.pos, TOP_MARGIN, screen_height() - PLAYER_SIZE - BOTTOM_MARGIN);
}

fn clampx(pos: Vec2, min: f32, max: f32) -> Vec2 {
    let mut new_pos = pos.clone();
    if pos.x() < min {
        new_pos.set_x(min);
    }
    if pos.x() > max {
        new_pos.set_x(max);
    }

    return new_pos;
}

fn clampy(pos: Vec2, min: f32, max: f32) -> Vec2 {
    let mut new_pos = pos.clone();
    if pos.y() < min {
        new_pos.set_y(min);
    }
    if pos.y() > max {
        new_pos.set_y(max);
    }

    return new_pos;
}

fn draw_player(player: Player, ship_tx: Texture2D) {
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

