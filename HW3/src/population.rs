pub mod population {
    use crate::chromosome::chromosome::Chromosome;
    use crate::utils::utils::{get_random_number_range, Point, calculate_dist};
    use std::collections::HashMap;
    use std::f32::MAX;

    pub struct Population {
        chromosomes: Vec<Chromosome>,
        population_size: usize,
        elitism_percentage: f32,
        cities_dist: Vec<Vec<f32>>,
    }

    impl Population {
        pub fn new(genes_size: usize, population_size: usize, cities: HashMap<i32, Point>, elitism_percentage: f32) -> Self {
            let mut temp: Vec<Chromosome> = vec!();
            let gene_pool: Vec<i32> = cities.keys().map(|&x| x).collect();
            for _ in 0..population_size {
                temp.push(Chromosome::new_random(genes_size, &gene_pool));
            }
            let cities_dist = calculate_dist(&cities);
            Population {
                chromosomes: temp,
                population_size,
                elitism_percentage,
                cities_dist,
            }
        }
        pub fn selection(&mut self) {
            let cities_dist = self.cities_dist.clone();
            self.chromosomes.sort_by(|c1, c2|
                c1.calculate_fitness(&cities_dist).partial_cmp(&c2.calculate_fitness(&cities_dist)).unwrap()); // O(N*logN*S)

            let count: usize = (self.population_size as f32 * self.elitism_percentage) as usize;
            self.chromosomes.split_off(count).truncate(count);
        }

        pub fn crossovers(&self, parents: &Vec<Chromosome>) -> Vec<Chromosome> {
            let mut next_generation: Vec<Chromosome> = vec!();

            let target: usize = self.population_size - parents.len();

            while next_generation.len() < target {
                let (p1, p2): (usize, usize) = self.get_random_parents_indexes(&parents); //O(1*)

                next_generation.push(parents[p1].crossover(&parents[p2]));

                if next_generation.len() != target {
                    next_generation.push(parents[p2].crossover(&parents[p1]));
                }
            }
            next_generation
        }

        pub fn mutation(&mut self) {
            let chromosome_index = get_random_number_range(0, self.population_size);
            self.chromosomes[chromosome_index].mutate();
        }

        pub fn generation(&mut self) -> f32 {
            self.selection();
            let selected_parents: &Vec<Chromosome> = &self.chromosomes;

            let mut children: Vec<Chromosome> = self.crossovers(selected_parents);
            for parent in selected_parents {
                children.push(parent.clone());
            }
            self.chromosomes = children;

            self.mutation();

            let mut min_score: f32 = MAX;
            for chromosome in &self.chromosomes {
                let current_score = chromosome.calculate_fitness(&self.cities_dist);
                if min_score > current_score {
                    min_score = current_score;
                }
            }
            min_score
        }

        fn get_random_parents_indexes(&self, parents: &Vec<Chromosome>) -> (usize, usize) { // O(1*)
            let first: usize = get_random_number_range(0, parents.len());
            let second = {
                let mut temp = get_random_number_range(0, parents.len());
                while temp == first {
                    temp = get_random_number_range(0, parents.len());
                }
                temp
            };
            (first, second)
        }
    }
}