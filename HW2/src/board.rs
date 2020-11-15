pub mod board {
    use rand::Rng;

    fn remove_duplicate_queen(count: i32) -> i32{
        if count == 1{
            0
        }
        else{
            count - 1
        }
    }

    fn get_random_element_from_vec(vec: Vec<usize>) -> usize{
        let mut rng = rand::thread_rng();
        vec[rng.gen_range(0, vec.len())]
    }
    pub struct Board {
        pub board: Vec<usize>,
        pub row_conflicts: Vec<usize>,
        pub main_diagonal_conflicts: Vec<usize>,
        pub secondary_diagonal_conflicts: Vec<usize>,
        pub size: usize,
    }

    impl Board {
        pub fn new(size: usize) -> Self {
            let mut rng = rand::thread_rng();
            let mut temp = Board {
                board: vec![0; size],
                row_conflicts: vec![0; size],
                main_diagonal_conflicts: vec![0; 2*size - 1],
                secondary_diagonal_conflicts: vec![0; 2*size - 1],
                size,
            };

            for column in 0..size{
                let row = temp.get_new_position_for_queen(column);
                temp.put(row, column);
            }
            temp
        }
        pub fn new_full_random(size: usize) -> Self {
            let mut rng = rand::thread_rng();
            let mut temp = Board {
                board: vec![0; size],
                row_conflicts: vec![0; size],
                main_diagonal_conflicts: vec![0; 2*size - 1],
                secondary_diagonal_conflicts: vec![0; 2*size - 1],
                size,
            };

            for column in 0..size{
                let row = rng.gen_range(0, size);
                temp.put(row, column);
            }
            temp
        }
        pub fn new_non_random(size: usize, items: Vec<usize>) -> Self {
            Board {
                board: items,
                row_conflicts: (0..size).collect(),
                main_diagonal_conflicts: (0..2 * size - 1).collect(),
                secondary_diagonal_conflicts: (0..2 * size - 1).collect(),
                size
            }
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
        pub fn get_board_conflicts_count(&self) -> usize{
            self.board.iter().enumerate().filter(|(column, &row)| self.get_conflicts_at(row, *column) > 3).count()
        }

        pub fn iterate(&mut self){
            let queen_to_move: usize = self.get_queen_with_max_conflicts();
            let current_row: usize = self.board[queen_to_move];

            let new_row: usize = self.get_new_position_for_queen(queen_to_move);

            self.move_queen(current_row, queen_to_move, new_row);
        }

        fn get_new_position_for_queen(&self, column: usize) -> usize{
            let mut min_conflicts: Vec<usize> = vec!();
            let mut min_score: usize = std::usize::MAX;

            for row in 0..self.size{
                let current_score = self.get_conflicts_at(row, column);

                if current_score < min_score{
                    min_conflicts.clear();
                    min_conflicts.push(row);

                    min_score = current_score;
                }
                else if current_score == min_score{
                    min_conflicts.push(row)
                }
            }
            get_random_element_from_vec(min_conflicts)
        }
        fn get_queen_with_max_conflicts(&self) -> usize{
            let mut max_queens: Vec<usize> = vec!();
            let mut max_score = 0;
            for (column, &row) in self.board.iter().enumerate(){
                let current_score = self.get_conflicts_at(row, column);

                if current_score > max_score{
                    max_queens.clear();
                    max_queens.push(column);

                    max_score = current_score;
                }
                else if current_score == max_score{
                    max_queens.push(column)
                }
            }

            get_random_element_from_vec(max_queens)
        }
        fn get_conflicts_at(&self, row: usize, column: usize) -> usize{
            let main_diagonal_index: usize = self.get_main_diagonal_index(row, column);
            let secondary_diagonal_index: usize = self.get_secondary_diagonal_index(row, column);
            self.row_conflicts[row] + self.main_diagonal_conflicts[main_diagonal_index] + self.secondary_diagonal_conflicts[secondary_diagonal_index]
        }

        fn get_main_diagonal_index(&self, row: usize, column: usize) -> usize{
            column + self.size - 1 - row
        }
        fn get_secondary_diagonal_index(&self, row: usize, column: usize) -> usize{
            column + row
        }

        fn remove(&mut self, row: usize, column: usize){
            self.row_conflicts[row] -= 1;

            let main_diagonal_index: usize = self.get_main_diagonal_index(row, column);
            let secondary_diagonal_index: usize = self.get_secondary_diagonal_index(row, column);
            self.main_diagonal_conflicts[main_diagonal_index] -= 1;
            self.secondary_diagonal_conflicts[secondary_diagonal_index] -= 1;
        }
        fn put(&mut self, row: usize, column: usize){
            self.board[column] = row;
            self.row_conflicts[row] += 1;

            let main_diagonal_index: usize = self.get_main_diagonal_index(row, column);
            let secondary_diagonal_index: usize = self.get_secondary_diagonal_index(row, column);
            self.main_diagonal_conflicts[main_diagonal_index] += 1;
            self.secondary_diagonal_conflicts[secondary_diagonal_index] += 1;
        }
        fn move_queen(&mut self, old_row: usize, column: usize, new_row: usize){
            self.remove(old_row, column);
            self.put(new_row, column);
        }
    }
}