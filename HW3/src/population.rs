pub mod population{
    use crate::chromosome::chromosome::Chromosome;
    use crate::utils::utils::{get_random_number_range, tsp_fitness, Point, get_random_real_number};

    pub struct Population {
        chromosomes: Vec<Chromosome>,
        genes_size: usize,
        population_size: usize,
        children_percentage: f32
    }
    impl Population {
        pub fn new(genes_size: usize, population_size: usize, gene_pool: &Vec<Point>) -> Self{
            let mut temp: Vec<Chromosome> = vec!();

            for _ in 0..population_size{
                temp.push(Chromosome::new_random(genes_size, gene_pool));
            }
            Population {
                chromosomes: temp,
                genes_size,
                population_size,
                children_percentage: 0.7f32
            }
        }
        pub fn selection(&mut self) -> Vec<Chromosome>{
            self.eval_chromosomes();

            let mut sum: f32 = 0f32;
            for chromosome in &self.chromosomes{
                sum += chromosome.get_fitness();
            }

            let mut sum_of_probabilities: f32 = 0f32;
            let mut probability: Vec<f32> = vec![0f32; self.population_size];

            for (i, chromosome) in self.chromosomes.iter().enumerate(){
                probability[i] = sum_of_probabilities + (chromosome.get_fitness() / sum);
                sum_of_probabilities += probability[i];
            }

            let number: f32 = get_random_real_number();

            let mut chosen: Vec<Chromosome> = vec!();
            for (i, chromosome) in self.chromosomes.iter().enumerate(){
                if number > probability[i]{
                    chosen.push(chromosome.clone());
                }
            }
            chosen
        }

        pub fn generate_next_generation(&mut self, parents: Vec<Chromosome>) -> Vec<Chromosome>{
            let mut next_generation: Vec<Chromosome> = vec!();

            while next_generation.len() < (self.children_percentage * self.population_size as f32) as usize {
                let parents: [&Chromosome; 2] = self.get_random_parents();
                let children: [Chromosome; 2] = parents[0].crossover(parents[1]);

                next_generation.push(children[0].clone());
                next_generation.push(children[1].clone());
            }
            next_generation
        }

        pub fn mutation(&mut self){
            let chromosome_index = get_random_number_range(0, self.population_size);
            self.chromosomes[chromosome_index].mutate();
        }

        pub fn generation(&mut self) {
            let mut selected_parents: Vec<Chromosome> = self.selection();
            let mut children: Vec<Chromosome> = self.generate_next_generation(selected_parents);

            while children.len() < self.population_size {
                let index = get_random_number_range(0, self.chromosomes.len());
                children.push(self.chromosomes[index].clone());
            }
            self.chromosomes = children;
            self.mutation();

            self.eval_chromosomes();
            println!("{}", self.chromosomes[0].get_fitness())
        }

        pub fn get_chromosomes(&self) -> &Vec<Chromosome> {
            &self.chromosomes
        }
        fn eval_chromosomes(&mut self) {
            for mut chromosome in self.chromosomes.iter_mut(){
                (chromosome).calculate_fitness(tsp_fitness);
            }
        }
        fn generate_children_random_parents(&self) -> [Chromosome; 2]{
            let first = get_random_number_range(0, self.chromosomes.len());
            let second = {
                let mut temp = get_random_number_range(0, self.chromosomes.len());
                while temp == first{
                    temp = get_random_number_range(0, self.chromosomes.len());
                }
                temp
            };

            self.chromosomes[first].crossover(&self.chromosomes[second])
        }

        fn get_random_parents(&self) -> [&Chromosome; 2]{
            let first: usize = get_random_number_range(0, self.chromosomes.len());
            let second = {
                let mut temp = get_random_number_range(0, self.chromosomes.len());
                while temp == first{
                    temp = get_random_number_range(0, self.chromosomes.len());
                }
                temp
            };

            [&self.chromosomes[first], &self.chromosomes[second]]
        }
    }
}