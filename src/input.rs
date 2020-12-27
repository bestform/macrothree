use macroquad::prelude::*;
use crate::{PlayerMovementState};
use crate::PlayerMovementState::{LEFT, RIGHT, UP, DOWN, IDLE};
use crate::state::GameState;
use crate::menu::Menu;
use crate::menu::MenuState::CONTINUE;

pub fn handle_global_shortcuts(current_state: GameState) -> GameState {
    if is_key_down(KeyCode::Q) {
        return GameState::GameOver;
    }
    if is_key_down(KeyCode::Escape) {
        return GameState::Menu;
    }

    return current_state;
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

impl Menu {
    pub fn handle_click(&mut self, current_state: GameState) -> GameState {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            for menu_item in self.items.iter() {
                let dim = menu_item.dimensions();

                if x > dim.0 && x < dim.2 && y > dim.1 && y < dim.3 {
                    let new_gamestate = match menu_item.id {
                        0 => {
                            self.state = CONTINUE;
                            GameState::Playing
                        },
                        2 => GameState::GameOver,
                        _ => current_state
                    };

                    return new_gamestate;
                }
            }
        }

        current_state
    }

    pub fn handle_mouse_hover(&mut self) {
        let (x, y) = mouse_position();
        for menu_item in self.items.iter_mut() {
            menu_item.hover = false;
            let dim = menu_item.dimensions();

            if x > dim.0 && x < dim.2 && y > dim.1 && y < dim.3 {
                menu_item.hover = true;
            }
        }
    }
}
