pub mod chromosome {
    extern crate rand;

    pub static MAX_VALUE: usize = 1000;

    use crate::utils::utils::{Point, get_random_number_range};

    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use self::rand::Rng;

    pub struct Chromosome {
        genes: Vec<Point>,
        size: usize,
        fitness: f32
    }

    impl Chromosome {
        pub fn new(size: usize) -> Self{
            Chromosome{
                genes: vec![Point::new(MAX_VALUE, MAX_VALUE); size],
                size,
                fitness: -1f32
            }
        }
        pub fn new_random(size: usize, gene_pool: &Vec<Point>) -> Self{
            Chromosome{
                genes: {
                    let mut temp = gene_pool.clone();
                    temp.shuffle(&mut thread_rng());
                    temp
                },
                size,
                fitness: -1f32
            }
        }
        pub fn get_fitness(&self) -> f32{
            self.fitness
        }
        pub fn calculate_fitness(&mut self, fitness_function: fn(&Vec<Point>) -> f32) -> f32 {
            self.fitness = fitness_function(&self.genes);
            self.fitness
        }

        pub fn get_genes(&self) -> &Vec<Point> {
            &self.genes
        }
        pub fn crossover(&self, other: &Chromosome) -> [Chromosome; 2]{
            self.cycle_crossover(other)
        }

        pub fn mutate(&mut self){
            let a: usize = get_random_number_range(0, self.size);
            let b: usize = get_random_number_range(a, self.size);

            self.genes.swap(a, b);
        }

        pub fn get_size(&self) -> usize{
            self.size
        }

        fn search(&self, to_find: &Point) -> Option<usize>{
            for (index, point) in self.genes.iter().enumerate(){
                if point == to_find{
                    return Some(index)
                }
            }
            None
        }

        fn cycle_crossover(&self, other: &Chromosome) -> [Chromosome; 2] {
            let mut child1: Chromosome = Chromosome::new(self.size);
            let mut child2: Chromosome = Chromosome::new(self.size);

            child1.genes[0] = self.genes[0];
            child2.genes[0] = other.genes[0];
            let mut i = 0;

            while !child1.genes.contains(&other.genes[i]){
                let target: Point = other.genes[i];
                let j: usize = self.search(&target).unwrap();

                child1.genes[j] = self.genes[j];
                child2.genes[j] = other.genes[j];
                i = j;
            }

            for i in 0..self.size {
                if child1.genes[i] == Point::new(MAX_VALUE, MAX_VALUE) {
                    child1.genes[i] = other.genes[i];
                    child2.genes[i] = self.genes[i];
                }
            }
            [child1, child2]
        }
    }

    impl Clone for Chromosome{
        fn clone(&self) -> Self {
            Chromosome{
                genes: self.genes.clone(),
                size: self.size,
                fitness: self.fitness
            }
        }
    }
}