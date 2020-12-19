use crate::game::Game;
use crate::audio::Audio;

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
        let new_game_state = self.handle_player_move_for_frame();
        self.handle_create_for_frame(audio, frame_t);
        self.handle_movement_for_frame();
        self.handle_lifetimes_for_frame(frame_t);
        self.handle_collisions(audio);
        self.handle_points(frame_t);
        self.draw();

        return new_game_state;
    }
}