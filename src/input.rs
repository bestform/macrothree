use macroquad::prelude::*;
use std::process::exit;
use crate::{PlayerMovementState};
use crate::PlayerMovementState::{LEFT, RIGHT, UP, DOWN, IDLE};

pub fn handle_shortcuts() {
    if is_key_down(KeyCode::Q) {
        exit(0);
    }
}

pub fn handle_player_input() -> Vec<PlayerMovementState> {

    let mut player_movement_states = Vec::new();

    if !(is_key_down(KeyCode::A) && is_key_down(KeyCode::D)) {
        if is_key_down(KeyCode::A) {
            player_movement_states.push(LEFT);
        }
        if is_key_down(KeyCode::D) {
            player_movement_states.push(RIGHT);
        }
    }
    if !(is_key_down(KeyCode::W) && is_key_down(KeyCode::S)) {
        if is_key_down(KeyCode::W) {
            player_movement_states.push(UP);
        }
        if is_key_down(KeyCode::S) {
            player_movement_states.push(DOWN);
        }
    }

    if player_movement_states.len() == 0 {
        player_movement_states.push(IDLE);
    }

    player_movement_states
}
