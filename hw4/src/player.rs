pub mod player{
    use crate::board::board::{Board, Point};
    use crate::game::game::Game;

    pub enum PlayerType{
        Human,
        Bot
    }
    pub trait Player{
        fn play(&self, game: &Game) -> Point;
        fn get_board_value(&self) -> usize;
        fn get_board_symbol(&self) -> char;
        fn get_type(&self) -> PlayerType;
    }
}