use std::time::{Instant};
use std::io::{self};

use crate::board::board::Board;

mod board;

fn read_from_keyboard() -> i32{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<i32>(){
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

fn test(){
    let mut b: Board = Board::new_non_random(4, vec![2, 0, 3, 3]);
    b.compute_board();
    b.print();
    b.iterate();
    b.print();
}
fn main() {
    //test();

    println!("Enter n:");
    let n: i32 = read_from_keyboard();

    let now = Instant::now();
    let mut b: Board = Board::new(n);
    b.compute_board();

    let mut transforms: i32 = 0;
    while b.get_board_conflicts() != 0{
        b.iterate();
        //b.print();
        transforms += 1;
        if transforms > n{
            println!("Resetting the board");

            b = Board::new(n);
            b.compute_board();
            transforms = 0;
        }
    }
    if n < 50{
        b.print();
    }
    println!("Found answer in {} ms, after {} iterations", now.elapsed().as_millis(), transforms);
}
