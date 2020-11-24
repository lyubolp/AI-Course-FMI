pub mod board {
    use std::cmp::max;

    static EMPTY_FIELD_VALUE: u8 = 0;

    pub struct Point {
        x: u8,
        y: u8,
    }

    impl Point {
        pub fn new(x: u8, y: u8) -> Self {
            Point { x, y }
        }
    }

    pub struct Board {
        board: Vec<Vec<u8>>,
        board_size: usize,
    }

    impl Board {
        pub fn new(size: usize) -> Self {
            Board {
                board: vec![vec![EMPTY_FIELD_VALUE; size]; size],
                board_size: size,
            }
        }
        pub fn from_existing_with_move(old: &Board, new_move: Point, player_value: u8) -> Self {
            let mut new = Board::new(old.board_size);
            new.board = old.board.clone();
            new.play(new_move, player_value);
            new
        }

        pub fn is_board_win(&self, for_value: u8) -> bool {
            self.evaluate_rows(for_value) == 3
                || self.evaluate_cols(for_value) == 3
                || self.evaluate_main_diagonal(for_value) == 3
                || self.evaluate_secondary_diagonal(for_value) == 3
        }

        pub fn evaluate(&self, for_value: u8) -> usize {
            connected_to_value(self.evaluate_rows(for_value))
                + connected_to_value(self.evaluate_cols(for_value))
                + connected_to_value(self.evaluate_main_diagonal(for_value))
                + connected_to_value(self.evaluate_secondary_diagonal(for_value))
        }

        pub fn play(&mut self, location: Point, player_value: u8) {
            self.board[location.x][location.y] = player_value
        }

        pub fn get_next_states(&self, player_value: u8) -> Vec<(Board, Point)> {
            let mut states: Vec<(Board, Point)> = vec!();
            for free_space in self.get_free_spaces() {
                states.push((Board::from_existing_with_move(self, free_space, player_value), free_space);
            }
            states
        }

        fn get_free_spaces(&self) -> Vec<Point> {
            let mut free_spaces: Vec<Point> = vec!();
            for row in 0..self.board_size {
                for column in 0..self.board_size {
                    if self.board[row][column] == EMPTY_FIELD_VALUE {
                        free_spaces.push(Point::new(row as u8, column as u8));
                    }
                }
            }
            free_spaces
        }
        fn connected_to_value(&self, amount_connected: usize) -> usize {
            10 * amount_connected
        }
        fn evaluate_rows(&self, for_value: u8) -> usize {
            let mut max_val: usize = 0;
            self.board.iter().map(
                |row: &Vec<u8>|
                    max_val = max(max_val, row.iter().filter(|&&x| x == for_value).count())
            );
            connected_to_value(max_val)
        }
        fn evaluate_cols(&self, for_value: u8) -> usize {
            let mut max_val: usize = 0;
            (0..self.board_size).map(
                |column|
                    max_val = max(max_val, self.board.iter().filter(|&&row| row[column] == for_value).count())
            );
            connected_to_value(max_val)
        }
        fn evaluate_main_diagonal(&self, for_value: u8) -> usize {
            connected_to_value((0..self.board_size).filter(|&i| self.board[i][i] == for_value).count())
        }

        fn evaluate_secondary_diagonal(&self, for_value: u8) -> usize {
            connected_to_value((0..self.board_size).filter(|&i| self.board[i][self.board_size - i - 1] == for_value).count())
        }
    }
}