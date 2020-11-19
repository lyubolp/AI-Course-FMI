pub mod utils{
    use rand::Rng;

    pub fn get_random_number_range(from: usize, to: usize) -> usize{
        let mut rng = rand::thread_rng();
        rng.gen_range(from, to) as usize
    }

    pub fn get_random_real_number() -> f32{
        let mut rng = rand::thread_rng();
        rng.gen_range(0, 100) as f32 / 100f32
    }

    pub fn tsp_fitness(points: &Vec<Point>) -> f32{
        let mut result: f32 = 0f32;
        for i in 0..points.len() - 1{
            result += points[i].get_distance_to(&points[i+1]);
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
        pub fn new_random(MAX_VALUE: usize) -> Self{
            let mut rng = rand::thread_rng();
            Point{
                x: rng.gen_range(0, MAX_VALUE) as usize,
                y: rng.gen_range(0, MAX_VALUE) as usize
            }
        }
        pub fn get_distance_to(&self, other: &Point) -> f32 {
            let x_diff:f32 = other.x as f32 - self.x as f32;
            let y_diff:f32 = other.y as f32 - self.y as f32;
            (x_diff.powi(2) + y_diff.powi(2)).sqrt()
        }
    }


}