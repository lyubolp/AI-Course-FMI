pub mod bot {
    use crate::player::player::{Player, PlayerType};
    use crate::board::board::{Board, Point};
    use std::i32::{MIN, MAX};
    use std::cmp::{max, min};
    use crate::game::game::Game;

    pub struct Bot {
        board_value: usize,
        board_symbol: char,
    }

    impl Bot {
        pub fn new(board_value: usize, board_symbol: char) -> Self {
            Bot {
                board_value,
                board_symbol,
            }
        }
    }

    impl Player for Bot {
        fn play(&self, game: &Game) -> Point {
            game.minimax()
        }

        fn get_board_value(&self) -> usize {
            self.board_value
        }

        fn get_board_symbol(&self) -> char {
            self.board_symbol
        }

        fn get_type(&self) -> PlayerType {
            PlayerType::Bot
        }
    }
}