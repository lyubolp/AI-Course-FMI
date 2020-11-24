pub mod bot {
    use crate::player::player::{Player, PlayerType};
    use crate::board::board::{Board, Point};
    use std::usize::MIN;
    use std::cmp::min;

    pub struct Bot {
        board_value: u8,
        board_symbol: char,
    }

    impl Bot {
        pub fn new(board_value: u8, board_symbol: char) -> Self {
            Bot {
                board_value,
                board_symbol,
            }
        }
        fn maximize(&self, state: &Board, current_depth: u8) -> usize {
            if state.is_board_win(self.board_value) {
                state.evaluate(self.board_value)
            } else {
                state.get_next_states(
                    self.board_value).iter()
                    .map(|(next_state, _)| self.minimize(next_state, current_depth + 1))
                    .max()
                    .unwrap()
            }
        }

        fn minimize(&self, state: &Board, current_depth: u8) -> usize {
            if state.is_board_win(self.board_value) {
                state.evaluate(self.board_value) - current_depth
            } else {
                state.get_next_states(
                    self.board_value).iter()
                                                .map(|(next_state, _)| self.maximize(next_state, current_depth + 1))
                                                .min()
                                                .unwrap()
            }
        }
        fn minimax(&self, state: &Board) -> Point {
            let score = self.maximize(state, 0);

            for (next_state, next_move) in state.get_next_states(self.board_value){
                if next_state.evaluate(self.board_value) == score{
                    return next_move;
                }
            }
            panic!("Can't find a move");
        }
    }

    impl Player for Bot {
        fn play(&self, state: &Board) -> Point {
             self.minimax(state)
        }

        fn get_board_value(&self) -> u8 {
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