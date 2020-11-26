pub mod utils{
    use rand::Rng;
    use std::collections::HashMap;

    pub fn get_random_number_range(from: usize, to: usize) -> usize{
        let mut rng = rand::thread_rng();
        rng.gen_range(from, to) as usize
    }
    pub fn calculate_dist(cities: &HashMap<i32, Point>) -> Vec<Vec<f32>>{
        let mut result = vec!();

        for _ in 0..cities.len(){
            result.push(vec![0f32; cities.len()]);
        }

        for start in 0..cities.len(){
            for end in 0..cities.len(){
                if start != end{
                    result[start][end] = cities.get(&(start as i32)).unwrap().get_distance_to(cities.get(&(end as i32)).unwrap());
                }
            }
        }

        result
    }

    #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Point{
        x: usize,
        y: usize
    }

    impl Point{

        pub fn new(x: usize, y: usize) -> Self{
            Point{
                x, y
            }
        }
        pub fn get_distance_to(&self, other: &Point) -> f32 {
            let x_diff:f32 = other.x as f32 - self.x as f32;
            let y_diff:f32 = other.y as f32 - self.y as f32;
            (x_diff.powi(2) + y_diff.powi(2)).sqrt()
        }
    }


}