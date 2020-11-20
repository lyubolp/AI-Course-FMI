use crate::population::population::Population;
use crate::utils::utils::{Point, get_random_number_range};
use std::fs;
use std::collections::HashMap;

mod chromosome;
mod utils;
mod population;

fn read_input_from_file(filename: &str) -> HashMap<i32, Point>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut result: HashMap<i32, Point> = HashMap::new();
    let mut counter: i32 = 0;
    for line in contents.split('\n'){
        let line_vec: Vec<&str> = line.split(' ').collect();

        let first: f32 = line_vec[0].parse().unwrap();
        let second: f32 = line_vec[1].parse().unwrap();
        result.insert(counter, Point::new(first as usize, second as usize));
        counter += 1;
    }

    result
}

fn generate_random_cities(size: usize, (min, max): (usize, usize)) -> HashMap<i32, Point>{
    let mut result: HashMap<i32, Point> = HashMap::new();
    for i in 0..size{
        let first = get_random_number_range(min, max);
        let second: usize = get_random_number_range(min, max);

        result.insert(i as i32, Point::new(first, second));
    }

    result
}
fn main() {
    let MAX_GENERATIONS: i32 = 4000;
    let POPULATION_SIZE: usize = 400;
    let cities: HashMap<i32, Point> = read_input_from_file("Berlin52_2.txt");
    //let cities = generate_random_cities(30, (0, 100));

    let mut temp: Population = Population::new(cities.len(), POPULATION_SIZE, cities);

    for i in 0..MAX_GENERATIONS{
        println!("Generation {}'s score is {}", i, temp.generation());
    }
}
