use crate::game::Game;
use crate::audio::Audio;
use crate::menu::Menu;
use crate::input::handle_global_shortcuts;

pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub trait RunState {
    fn run(&mut self, frame_t: f64, audio: &mut Audio) -> GameState;
}

impl RunState for Game {
    fn run(&mut self, frame_t: f64, audio: &mut Audio) -> GameState {
        let new_game_state = handle_global_shortcuts(GameState::Playing);
        self.handle_player_move_for_frame();
        self.handle_create_for_frame(audio, frame_t);
        self.handle_movement_for_frame();
        self.handle_lifetimes_for_frame(frame_t);
        self.handle_collisions(audio);
        self.handle_points(frame_t);
        self.draw();

        return new_game_state;
    }
}

impl RunState for Menu {
    fn run(&mut self, _frame_t: f64, _audio: &mut Audio) -> GameState {
        let mut new_game_state = handle_global_shortcuts(GameState::Menu);
        self.handle_mouse_hover();
        new_game_state = self.handle_click(new_game_state);
        self.render();

        new_game_state
    }
}