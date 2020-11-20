pub mod population{
    use crate::chromosome::chromosome::Chromosome;
    use crate::utils::utils::{get_random_number_range, Point};
    use std::collections::HashMap;
    use std::cmp::min;

    pub struct Population {
        chromosomes: Vec<Chromosome>,
        genes_size: usize,
        population_size: usize,
        elitism_percentage: f32,
        cities: HashMap<i32, Point>
    }
    impl Population {
        pub fn new(genes_size: usize, population_size: usize, cities: HashMap<i32, Point>) -> Self{
            let mut temp: Vec<Chromosome> = vec!();
            let gene_pool: Vec<i32> = cities.keys().map(|&x| x).collect();
            for _ in 0..population_size{
                temp.push(Chromosome::new_random(genes_size, &gene_pool));
            }
            Population {
                chromosomes: temp,
                genes_size,
                population_size,
                elitism_percentage: 0.1f32,
                cities
            }
        }
        pub fn selection(&mut self) -> Vec<Chromosome>{
            let mut cloned = self.chromosomes.clone();
            cloned.sort_by(|c1, c2|
                c1.calculate_fitness(&self.cities).partial_cmp(&c2.calculate_fitness(&self.cities)).unwrap());

            let count: usize = (self.population_size as f32 * self.elitism_percentage) as usize;
            cloned[0..count].to_vec()
        }

        pub fn crossovers(&mut self, parents: &mut Vec<Chromosome>) -> Vec<Chromosome>{
            let mut next_generation: Vec<Chromosome> = vec!();

            let target: usize = self.population_size - parents.len();

            while next_generation.len() < target {
                let (p1, p2): (usize, usize) = self.get_random_parents_indexes(&parents);

                let c1: Chromosome = parents[p1].crossover(&parents[p2]);
                next_generation.push(c1.clone());

                if next_generation.len() != target{
                    let c2: Chromosome = parents[p2].crossover(&parents[p1]);
                    next_generation.push(c2.clone());
                }
            }
            next_generation
        }

        pub fn mutation(&mut self){
            let chromosome_index = get_random_number_range(0, self.population_size);
            self.chromosomes[chromosome_index].mutate();
        }

        pub fn generation(&mut self) -> f32{
            let mut selected_parents: Vec<Chromosome> = self.selection();
            let children: Vec<Chromosome> = self.crossovers(&mut selected_parents);

            self.chromosomes = children;
            self.chromosomes.append(&mut selected_parents);

            self.mutation();

            let mut min_score: f32 = self.chromosomes[0].calculate_fitness(&self.cities);
            min_score
        }

        pub fn get_chromosomes(&self) -> &Vec<Chromosome> {
            &self.chromosomes
        }

        fn get_random_parents_indexes(&self, parents: &Vec<Chromosome>) -> (usize, usize){
            let first: usize = get_random_number_range(0, parents.len());
            let second = {
                let mut temp = get_random_number_range(0, parents.len());
                while temp == first{
                    temp = get_random_number_range(0, parents.len());
                }
                temp
            };
            (first, second)
        }
    }
}