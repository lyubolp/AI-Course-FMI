use crate::population::population::Population;
use crate::utils::utils::Point;
use crate::chromosome::chromosome::Chromosome;
use std::fs;

mod chromosome;
mod utils;
mod population;

fn read_input_from_file(filename: &str) -> Vec<Point>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut result: Vec<Point> = vec!();
    for line in contents.split('\n'){
        let line_vec: Vec<&str> = line.split(' ').collect();

        let first: f32 = line_vec[0].parse().unwrap();
        let second: f32 = line_vec[1].parse().unwrap();
        result.push(Point::new(first as usize, second as usize))
    }

    result
}
fn main() {
    let MAX_GENERATIONS: i32 = 4000;
    let cities: Vec<Point> = read_input_from_file("Berlin52.txt");

    let mut temp: Population = Population::new(cities.len(), 100, &cities);

    for _ in 0..MAX_GENERATIONS{
        temp.generation();
    }
    //test();
}
