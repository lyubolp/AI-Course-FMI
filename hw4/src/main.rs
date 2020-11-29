use crate::game::game::Game;
use crate::board::board::{Board, Point};

mod board;
mod player;
mod bot;
mod human_player;
mod game;

fn main() {
    let mut game: Game = Game::new(true, 3);
    game.play();
}
