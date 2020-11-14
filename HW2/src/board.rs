pub mod board {
    use rand::Rng;
    use std::i32::MAX;

    fn remove_duplicate_queen(count: i32) -> i32{
        if count == 1{
            0
        }
        else{
            count - 1
        }
    }

    fn choose_from_moves(moves: Vec<i32>) -> i32{
        if moves.len() == 1 {
            moves[0]
        }
        else{
            let mut rng = rand::thread_rng();
            moves[rng.gen_range(0, moves.len())]
        }
    }
    pub struct Board {
        pub board: Vec<i32>,
        pub row_conflicts: Vec<i32>,
        pub diagonal_conflicts_main: Vec<i32>,
        pub diagonal_conflicts_secondary: Vec<i32>,
        pub size: i32,
    }

    impl Board {
        pub fn new(size: i32) -> Self {
            let mut rng = rand::thread_rng();
            Board {
                board: (0..size).map(|_| rng.gen_range(0, size)).collect(),
                row_conflicts: (0..size).collect(),
                diagonal_conflicts_main: (0..2*size - 1).collect(),
                diagonal_conflicts_secondary: (0..2*size - 1).collect(),
                size,
            }
        }
        pub fn new_non_random(size: i32, items: Vec<i32>) -> Self{
            Board{
                board: items,
                row_conflicts: (0..size).collect(),
                diagonal_conflicts_main: (0..2*size - 1).collect(),
                diagonal_conflicts_secondary: (0..2*size - 1).collect(),
                size
            }
        }

        pub fn compute_board(&mut self){
            for row in 0..self.size{
                self.row_conflicts[row as usize] = self.compute_conflicts_in_row(row);
            }
            for diagonal_second in 0..self.size{
                let (first_row, first_column, first_target): (i32, i32, i32) = (diagonal_second, 0, -diagonal_second);
                let (second_row, second_column, second_target): (i32, i32, i32) = (0, diagonal_second,  diagonal_second);
                let (third_row, third_column, third_target): (i32, i32, i32) = (0, diagonal_second, diagonal_second);
                let (forth_row, forth_column, forth_target): (i32, i32, i32) = (diagonal_second, self.size - 1, diagonal_second + self.size - 1);

                let main_diagonal_first_index: usize = self.get_main_diagonal_index_from(first_row, first_column) as usize;
                let main_diagonal_second_index: usize = self.get_main_diagonal_index_from(second_row, second_column) as usize;
                let secondary_diagonal_first_index: usize = self.get_secondary_diagonal_index_from(third_row, third_column) as usize;
                let secondary_diagonal_second_index: usize = self.get_secondary_diagonal_index_from(forth_row, forth_column) as usize;

                self.diagonal_conflicts_main[main_diagonal_first_index] = self.compute_conflicts_in_diagonal(first_target, |x,y| y - x);
                self.diagonal_conflicts_main[main_diagonal_second_index] = self.compute_conflicts_in_diagonal(second_target, |x,y| y - x);
                self.diagonal_conflicts_secondary[secondary_diagonal_first_index] = self.compute_conflicts_in_diagonal(third_target, |x,y| y + x);
                self.diagonal_conflicts_secondary[secondary_diagonal_second_index] = self.compute_conflicts_in_diagonal(forth_target, |x, y| y + x);
            }
        }
        pub fn update_conflicts(&mut self, (from_row, from_column): (i32, i32), to_row: i32){
            self.row_conflicts[from_row as usize] -= 1;
            self.row_conflicts[to_row as usize] += 1;

            let main_diagonal_index_from: usize = self.get_main_diagonal_index_from(from_row, from_column) as usize;
            let main_diagonal_index_to: usize = self.get_main_diagonal_index_from(to_row, from_column) as usize;
            self.diagonal_conflicts_main[main_diagonal_index_from] -= 1;
            self.diagonal_conflicts_main[main_diagonal_index_to] += 1;

            let secondary_diagonal_index_from: usize = self.get_secondary_diagonal_index_from(from_row, from_column) as usize;
            let secondary_diagonal_index_to: usize = self.get_secondary_diagonal_index_from(to_row, from_column) as usize;

            self.diagonal_conflicts_secondary[secondary_diagonal_index_from] -= 1;
            self.diagonal_conflicts_secondary[secondary_diagonal_index_to] += 1;
        }

        pub fn find_queen_to_move(&self) -> i32{
            let mut max_scores: Vec<i32> = vec!();
            let mut max_score: i32 = 0;

            for (column, &row) in self.board.iter().enumerate(){
                let current_score: i32 = self.calculate_score_of_cell(row, column as i32);
                if max_score < current_score{
                    max_score = current_score;

                    max_scores.clear();
                    max_scores.push(column as i32)
                }
                else if max_score == current_score{
                    max_scores.push(column as i32);
                }
            }
            choose_from_moves(max_scores)
        }
        pub fn print(&self){
            for row in 0..self.size{
                for column in 0..self.size{
                    match self.board[column as usize] == row{
                        true => print!("* "),
                        false => print!("_ ")
                    }
                }
                print!("\n");
            }
            print!("\n");
        }
        
        pub fn iterate(&mut self){
            let start_column: i32 = self.find_queen_to_move();
            let start_row: i32 = self.board[start_column as usize];

            let mut min_score: i32 = self.calculate_score_of_cell(start_row, start_column);
            let mut min_scores: Vec<i32> = vec!();
            let mut min_row: i32 = start_row;

            for row in 0..self.size{
                let current_score = self.calculate_score_of_cell(row, start_column);
                if current_score < min_score{
                    min_score = current_score;
                    min_row = row;

                    min_scores.clear();
                    min_scores.push(row);
                }
                else if current_score == min_score{
                    min_scores.push(row);
                }
            }

            if min_scores.len() != 1{
                let mut rng = rand::thread_rng();
                min_row = min_scores[rng.gen_range(0, min_scores.len())];
            }
            self.board[start_column as usize] = min_row;
            self.update_conflicts((start_row, start_column), min_row);
        }

        pub fn get_board_conflicts(&self) -> i32{
            let mut score = 0;
            for (column, &row) in self.board.iter().enumerate(){
                score += self.calculate_score_of_cell(row, column as i32);
            }
            score
        }


        fn calculate_score_of_cell(&self, row: i32, column: i32) -> i32{
            self.get_row_conflicts_from(row) + self.get_main_diagonal_conflicts_from(row, column) + self.get_secondary_diagonal_conflicts_from(row, column)
        }

        fn get_row_conflicts_from(&self, row: i32) -> i32{
            remove_duplicate_queen(self.row_conflicts[row as usize])
        }
        fn get_main_diagonal_conflicts_from(&self, row: i32, column: i32) -> i32{
            let main_diagonal_index: i32 = self.get_main_diagonal_index_from(row, column);
            remove_duplicate_queen(self.diagonal_conflicts_main[main_diagonal_index as usize])
        }
        fn get_secondary_diagonal_conflicts_from(&self, row: i32, column: i32) -> i32{
            let secondary_diagonal_index: i32 = self.get_secondary_diagonal_index_from(row, column);
            remove_duplicate_queen(self.diagonal_conflicts_secondary[secondary_diagonal_index as usize])
        }


        fn get_main_diagonal_index_from(&self, row: i32, column: i32) -> i32{
            column - row + self.size - 1
        }
        fn get_secondary_diagonal_index_from(&self, row: i32, column: i32) -> i32{
            column + row
        }

        fn compute_conflicts_in_row(&self, row: i32) -> i32 {
            self.board.iter().filter(|&&x| x == row).fold(0, |acc, _| acc + 1)
        }
        fn compute_conflicts_in_diagonal(&self, target: i32, rule: fn(i32, i32) -> i32) -> i32{
            let mut result: i32 = 0;
            for i in 0..self.size{
                for j in 0..self.size{
                    if rule(i, j) == target && self.board[j as usize] == i{
                        result += 1;
                    }
                }
            }
            result
        }
    }
}