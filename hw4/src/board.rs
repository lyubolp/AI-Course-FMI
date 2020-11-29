pub mod board {
    use crate::player::player::Player;

    static EMPTY_FIELD_VALUE: usize = 0;

    #[derive(Clone, Copy, Debug)]
    pub struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        pub fn new(x: usize, y: usize) -> Self {
            Point { x, y }
        }
    }

    pub struct Board {
        board: Vec<Vec<usize>>,
        board_size: usize,
        parent: Option<Point>
    }

    impl Board {
        pub fn new(size: usize) -> Self {
            Board {
                board: vec![vec![EMPTY_FIELD_VALUE; size]; size],
                board_size: size,
                parent: None
            }
        }

        pub fn new_from_existing(board: Vec<Vec<usize>>, board_size: usize) -> Self{
            Board{
                board,
                board_size,
                parent: None
            }
        }
        pub fn from_existing_with_move(old: &Board, new_move: Point, player_value: usize) -> Self {
            let mut new = Board::new(old.board_size);
            new.board = old.board.clone();
            new.play(new_move, player_value);
            new.parent = Some(new_move);
            new
        }

        pub fn is_board_win(&self, players_values: (usize, usize)) -> bool{
            self.is_board_win_for(players_values.0) || self.is_board_win_for(players_values.1)
        }
        pub fn is_board_win_for(&self, for_value: usize) -> bool {
            self.evaluate_rows(for_value) == 3
                || self.evaluate_cols(for_value) == 3
                || self.evaluate_main_diagonal(for_value) == 3
                || self.evaluate_secondary_diagonal(for_value) == 3
        }

        pub fn evaluate(&self, (human_value, bot_value): (usize, usize)) -> i32 {
            if self.is_board_win_for(human_value){
                -100
            } else if self.is_board_win_for(bot_value){
                100
            } else{
                0
            }
        }

        pub fn play(&mut self, location: Point, player_value: usize) {
            self.board[location.x][location.y] = player_value
        }

        pub fn is_board_full(&self) -> bool {
            self.board.iter().flatten().filter(|&&x| x == EMPTY_FIELD_VALUE).count() == 0
        }

        pub fn get_next_states(&self, player_value: usize) -> Vec<(Board, Point)> {
            let mut states: Vec<(Board, Point)> = vec!();
            for free_space in self.get_free_spaces() {
                states.push((Board::from_existing_with_move(self, free_space, player_value), free_space));
            }
            states
        }

        pub fn print_board(&self, player1: &Box<dyn Player>, player2: &Box<dyn Player>){
            for row in &self.board{
                for &item in row{
                    if item == player1.get_board_value(){
                        print!("{} ", player1.get_board_symbol());
                    }
                    else if item == player2.get_board_value(){
                        print!("{} ", player2.get_board_symbol());
                    }
                    else{
                        print!("_ ");
                    }
                }
                print!("\n");
            }
        }

        pub fn is_field_taken(&self, field: Point) -> bool{
            self.board[field.x][field.y] != EMPTY_FIELD_VALUE
        }

        pub fn get_parent_move(&self) -> Option<Point> {
            self.parent
        }
        fn get_free_spaces(&self) -> Vec<Point> {
            let mut free_spaces: Vec<Point> = vec!();
            for x in 0..self.board_size {
                for y in 0..self.board_size {
                    if self.board[x][y] == EMPTY_FIELD_VALUE {
                        free_spaces.push(Point::new(x, y));
                    }
                }
            }
            free_spaces
        }
        fn connected_to_value(&self, amount_connected: i32) -> i32 {
            10i32.pow((amount_connected - 1) as u32)
        }
        fn evaluate_rows(&self, for_value: usize) -> i32 {
            self.board.iter().map(
                |row: &Vec<usize>| row.iter().filter(|&&x| x == for_value).count()
            ).max().unwrap() as i32
        }
        fn evaluate_cols(&self, for_value: usize) -> i32 {
            (0..self.board_size).map(
                |column| self.board.iter().filter(|row| row[column] == for_value).count()
            ).max().unwrap() as i32
        }
        fn evaluate_main_diagonal(&self, for_value: usize) -> i32 {
            (0..self.board_size).filter(|&i| self.board[i][i] == for_value).count() as i32
        }

        fn evaluate_secondary_diagonal(&self, for_value: usize) -> i32 {
            (0..self.board_size).filter(|&i| self.board[i][self.board_size - i - 1] == for_value).count() as i32
        }
    }
}