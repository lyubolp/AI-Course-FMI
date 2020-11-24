pub mod player{
    use crate::board::board::{Board, Point};

    pub enum PlayerType{
        Human,
        Bot
    }
    pub trait Player{
        fn play(&self, state: &Board) -> Point;
        fn get_board_value(&self) -> u8;
        fn get_board_symbol(&self) -> char;
        fn get_type(&self) -> PlayerType;
    }
}