use std::time::{Instant};
use std::io::{self};

use crate::board::board::Board;

mod board;

fn read_from_keyboard() -> usize{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<usize>(){
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, setting default value = 5");
                    5
                }
            }
        }
        Err(error) => {
            println!("Input error: {}, setting default value = 5", error);
            5
        }
    }
}

fn main() {
    println!("Enter n:");
    let n: usize = read_from_keyboard();

    let now = Instant::now();
    let mut b: Board = Board::new(n);

    let mut transforms: usize = 0;
    while b.get_board_conflicts_count() != 0{
        b.iterate();
        transforms += 1;
    }
    if n < 50{
        b.print();
    }
    println!("Found answer in {} ms, after {} iterations", now.elapsed().as_millis(), transforms);
}
