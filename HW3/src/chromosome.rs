pub mod chromosome {
    extern crate rand;
    use crate::utils::utils::{get_random_number_range};

    use rand::thread_rng;
    use rand::seq::SliceRandom;

    pub struct Chromosome {
        genes: Vec<i32>,
        size: usize
    }

    impl Chromosome {
        pub fn new(size: usize) -> Self{
            Chromosome{
                genes: vec![-1; size],
                size
            }
        }
        pub fn new_random(size: usize, gene_pool: &Vec<i32>) -> Self{
            Chromosome{
                genes: {
                    let mut temp = gene_pool.clone();
                    temp.shuffle(&mut thread_rng());
                    temp
                },
                size
            }
        }

        pub fn calculate_fitness(&self, dist: &Vec<Vec<f32>>) -> f32 { // O(S)
            let mut fitness: f32 = 0f32;
            for i in 0..self.size - 1{
                let first_city_id: usize = self.genes[i] as usize;
                let second_city_id: usize = self.genes[i+1] as usize;

                fitness += dist[first_city_id][second_city_id];
            }
            fitness
        }
        pub fn crossover(&self, other: &Chromosome) -> Chromosome {
            let mut child1: Chromosome = Chromosome::new(self.size);
            child1.genes[0] = self.genes[0];

            let mut i = 0;
            while !child1.genes.contains(&other.genes[i]){
                let target: i32 = other.genes[i];
                let j: usize = self.search(target).unwrap();

                child1.genes[j] = self.genes[j];
                i = j;
            }

            for i in 0..self.size {
                if child1.genes[i] == -1 {
                    child1.genes[i] = other.genes[i];
                }
            }
            child1
        }

        pub fn mutate(&mut self){
            let a: usize = get_random_number_range(0, self.size);
            let b: usize = get_random_number_range(0, self.size);

            self.genes.swap(a, b);
        }

        fn search(&self, to_find: i32) -> Option<usize>{
            for (index,&gene) in self.genes.iter().enumerate(){
                if gene == to_find{
                    return Some(index)
                }
            }
            None
        }
    }

    impl Clone for Chromosome{
        fn clone(&self) -> Self {
            Chromosome{
                genes: self.genes.clone(),
                size: self.size
            }
        }
    }
}