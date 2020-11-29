pub mod game {
    use std::io;
    use crate::board::board::{Board, Point};
    use crate::player::player::{Player, PlayerType};
    use crate::human_player::human_player::HumanPlayer;
    use crate::bot::bot::Bot;
    use std::i32::{MAX, MIN};
    use std::cmp::{min, max};

    pub fn read_from_keyboard() -> Option<Point> {
        let mut line_input = String::new();
        match io::stdin().read_line(&mut line_input) {
            Ok(_) => {
                let inputs: Vec<&str> = line_input.split(' ').collect();
                if inputs.len() != 2 {
                    println!("Invalid input format. Please enter two numbers, separated by a space.");
                    None
                } else {
                    let x = match inputs[0].trim().parse::<usize>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input format. Please enter two numbers, separated by a space.");
                            return None;
                        }
                    };
                    let y = match inputs[1].trim().parse::<usize>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input format. Please enter two numbers, separated by a space.");
                            return None;
                        }
                    };
                    Some(Point::new(x - 1, y - 1))
                }
            }
            Err(_) => {
                println!("Invalid input format. Please enter two numbers, separated by a space.");
                None
            }
        }
    }

    pub struct Game {
        board: Board,
        players: Vec<Box<dyn Player>>,
    }

    impl Game {
        pub fn new(is_player_first: bool, size: usize) -> Self {
            Game {
                board: Board::new(size),
                players: {
                    if is_player_first {
                        vec![Box::new(HumanPlayer::new(1, 'X')),
                             Box::new(Bot::new(2, 'O'))]
                    } else {
                        vec![Box::new(Bot::new(2, 'O')),
                             Box::new(HumanPlayer::new(1, 'X'))]
                    }
                },
            }
        }

        pub fn new_from_existing_board(board: Board) -> Self {
            Game {
                board,
                players: vec![Box::new(HumanPlayer::new(1, 'X')),
                              Box::new(Bot::new(2, 'O'))],
            }
        }

        pub fn play(&mut self) {
            let players_values: (usize, usize) = self.get_players_values();
            while !(self.board.is_board_win(players_values) || self.board.is_board_full()) {
                println!("Player {}'s turn:", self.players[0].get_board_symbol());
                self.board.print_board(&self.players[0], &self.players[1]);
                let player_move: Point = self.players[0].play(self);
                self.board.play(player_move, self.players[0].get_board_value());

                if self.board.is_board_win(players_values) || self.board.is_board_full() {
                    break;
                }
                self.board.print_board(&self.players[0], &self.players[1]);

                println!("Player {}'s turn:", self.players[1].get_board_symbol());
                let player_move: Point = self.players[1].play(self);
                self.board.play(player_move, self.players[1].get_board_value());
            }
            self.board.print_board(&self.players[0], &self.players[1]);
        }

        pub fn get_board(&self) -> &Board {
            &self.board
        }
        pub fn minimax(&self) -> Point {
            let score = self.maximize(&self.board, 0, (MIN, MAX));
            println!("Chosen score is {}, with move {:?}", score.0, score.1);
            score.1
        }

        fn get_human_bot_value(&self) -> (usize, usize) {
            if self.get_bot_id() == 0 {
                (self.players[1].get_board_value(), self.players[0].get_board_value())
            } else {
                (self.players[0].get_board_value(), self.players[1].get_board_value())
            }
        }

        fn minimize(&self, state: &Board, current_depth: i32, (a, mut b): (i32, i32)) -> (i32, Point) {
            let (human_value, bot_value): (usize, usize) = self.get_human_bot_value();
            if state.is_board_full() || state.is_board_win((human_value, bot_value)) {
                (state.evaluate((human_value, bot_value), current_depth), state.get_parent_move().unwrap())
            } else {
                let mut value: i32 = MAX;
                let mut current_move: Point = Point::new(0, 0);
                for (next_state, next_move) in state.get_next_states(human_value) {
                    let current_value = self.maximize(&next_state, current_depth + 1, (a, b)).0;

                    if current_value < value{
                        value = current_value;
                        current_move = next_move;
                    }

                    b = min(b, value);
                    if b <= a {
                        break;
                    }
                }
                (value, current_move)
            }
        }

        fn maximize(&self, state: &Board, current_depth: i32, (mut a, b): (i32, i32)) -> (i32, Point) {
            let (human_value, bot_value): (usize, usize) = self.get_human_bot_value();
            if state.is_board_full() || state.is_board_win((human_value, bot_value)) {
                (state.evaluate((human_value, bot_value), current_depth), state.get_parent_move().unwrap())
            } else {
                let mut value: i32 = MIN;
                let mut current_move: Point = Point::new(0, 0);
                for (next_state, next_move) in state.get_next_states(bot_value) {
                    let current_value = self.minimize(&next_state, current_depth + 1, (a, b)).0;

                    if current_value > value{
                        value = current_value;
                        current_move = next_move;
                    }

                    a = max(a, value);
                    if a >= b{
                        break;
                    }
                }
                (value, current_move)
            }
        }

        fn get_bot_id(&self) -> usize {
            match self.players[0].get_type() {
                PlayerType::Human => 1,
                PlayerType::Bot => 0
            }
        }
        fn get_players_values(&self) -> (usize, usize) {
            (self.players[0].get_board_value(), self.players[1].get_board_value())
        }
    }
}