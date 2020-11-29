pub mod human_player{
    use crate::player::player::{Player, PlayerType};
    use crate::board::board::{Board, Point};
    use crate::game::game::{read_from_keyboard, Game};

    pub struct HumanPlayer{
        board_value: usize,
        board_symbol: char
    }

    impl HumanPlayer{
        pub fn new(board_value: usize, board_symbol: char) -> Self {
            HumanPlayer {
                board_value,
                board_symbol,
            }
        }

        fn keyboard_input(&self) -> Point{
            let mut player_move = read_from_keyboard();
            loop{
                match player_move{
                    Some(m) => return m,
                    _ => ()
                }
            }
        }
    }

    impl Player for HumanPlayer{
        fn play(&self, game: &Game) -> Point {
            let mut player_move = self.keyboard_input();
            while game.get_board().is_field_taken(player_move) {
                player_move = self.keyboard_input();
            }
            player_move
        }

        fn get_board_value(&self) -> usize {
            self.board_value
        }

        fn get_board_symbol(&self) -> char {
            self.board_symbol
        }

        fn get_type(&self) -> PlayerType {
            PlayerType::Human
        }
    }
}