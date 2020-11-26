use crate::population::population::Population;
use crate::utils::utils::{Point, get_random_number_range};
use std::collections::HashMap;
use std::time::Instant;

mod chromosome;
mod utils;
mod population;

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
    let max_generations: i32 = 1500;
    let max_epoch: i32 = 20;
    let population_size: usize = 1600;
    let elitism: f32 = 0.2f32;
    let n: usize = 100;
    let print_points: [i32; 5] = [10, (max_generations / 4), 2 * (max_generations / 4), 3 * (max_generations / 4), max_generations - 1];
    let cities = generate_random_cities(n, (0, 100));

    let mut temp: Population;

    for epoch in 0..max_epoch{
        temp = Population::new(cities.len(), population_size, cities.clone(), elitism);

        let now = Instant::now();
        for generation in 0..max_generations {
            if print_points.contains(&generation){
                println!("Epoch {}, generation {}'s score is {}", epoch, generation, temp.generation());
            }
            else{
                temp.generation();
            }
        }
        println!("Epoch took {}s", now.elapsed().as_secs());
    }
}
