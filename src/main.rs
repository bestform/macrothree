use macroquad::prelude::*;

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
struct Star {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    brightness: f32,
}

#[derive(Clone)]
struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    alive: bool,
}

#[derive(Clone, Copy)]
struct Player {
    pos: Vec2,
    vel: Vec2,
    last_shot: f64
}

const PLAYER_WIDTH:f32 = 50.;
const PLAYER_HEIGHT:f32 = 30.;
const PLAYER_SPEED: f32 = 4.;
const BULLET_SPEED: f32 = -6.;
const BULLET_SIZE: f32 = 3.;
const STAR_DENSITY: f64 = 0.5;
#[macroquad::main("MacroThree")]
async fn main() {

    let mut player = Player {
        pos: Vec2::new(screen_width() / 2., screen_height() - 40.),
        vel: Vec2::new(0., 0.),
        last_shot: get_time()
    };

    let mut bullets:Vec<Bullet> = Vec::new();
    let mut stars:Vec<Star> = Vec::new();

    loop {
        let frame_t = get_time();
        // MOVE PLAYER
        handle_player_movement(&mut player);
        create_boolets(&mut player, &mut bullets, frame_t);
        handle_bullet_move(&mut bullets);
        handle_bullet_lifetime(&mut bullets, frame_t);
        create_stars(&mut stars, frame_t);
        handle_stars_move(&mut stars);
        handle_stars_lifetime(&mut stars);


        // DRAW
        clear_background(BLACK);
        draw_stars(stars.clone());
        draw_player(player);
        draw_bullets(bullets.clone());
        next_frame().await
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
            vel: Vec2::new(0., rand::gen_range(1., 1.5)),
            size: rand::gen_range(1., 2.),
            brightness: rand::gen_range(0.1, 1.)
        })
    }
}

fn handle_bullet_lifetime(bullets: &mut Vec<Bullet>, frame_t: f64) {
    bullets.retain(|bullet| bullet.shot_at + 2.5 > frame_t);
}

fn handle_bullet_move(bullets: &mut Vec<Bullet>) {
    for bullet in bullets {
        bullet.pos += bullet.vel;
    }
}

fn draw_bullets(bullets: Vec<Bullet>) {
    for bullet in bullets {
        draw_circle(bullet.pos.x(), bullet.pos.y(), BULLET_SIZE, LIGHTGRAY);
    }
}

fn create_boolets(player: &mut Player, bullets: &mut Vec<Bullet>, frame_t: f64) {
    if is_key_down(KeyCode::Space) && frame_t - player.last_shot > 0.1 {
        bullets.push(Bullet{
            pos: player.pos,
            vel: Vec2::new(0., BULLET_SPEED),
            shot_at: frame_t,
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

    player.pos += player.vel;
    player.pos = clampx(player.pos, 0., screen_width());
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

fn draw_player(player: Player) {
    draw_triangle(
        Vec2::new(player.pos.x() -1. * PLAYER_WIDTH/2., screen_height()),
        Vec2::new(player.pos.x() + PLAYER_WIDTH/2., screen_height()),
        Vec2::new(player.pos.x(), screen_height() - PLAYER_HEIGHT),
        LIGHTGRAY
    );
}
