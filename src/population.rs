use crate::dna::DNA;
use crate::pool::MatingPool;
use rand::prelude::*;

const POP_SIZE: i32 = 100;

pub struct Population {
    pops: Vec<DNA>
}

impl Population {
    pub fn new(target: &String) -> Population {
        let mut population: Vec<DNA> = vec![];
        for _i in 0..POP_SIZE {
            population.push(DNA::new(target));
        }
        Population { pops: population }
    }

    pub fn calculate_total_fitness(&mut self, target: &String) {
        self.pops.iter_mut().for_each(|pop| {
            pop.calculate_fitness(&target);
        })
    }

    pub fn find_best(&self) -> &DNA {
        if let Some(best) = self
            .pops
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).expect("expected a dna"))
        {
            best
        } else {
            panic!("didn't find dna")
        }
    }


    pub fn total_fitness(&self) -> f32{
        let mut total_fitness = 0.0;
        self.pops.iter().for_each(| dna | {
            total_fitness += dna.fitness;
        });

        total_fitness = total_fitness / self.pops.len() as f32;

        total_fitness

    }

    /// ## get_mating_pool
    /// gets a list of a randomly selected population
    /// but based on thier `fitness` they will get more chance to be in the bool
    pub fn get_mating_pool(&self) -> MatingPool{
        let mut pool: Vec<DNA> = vec![];
        let mut rng = thread_rng();
        while pool.len() < 10 {
            for dna in &self.pops {
                let chance = rng.gen_range(0.0..=1.0);
                if dna.fitness >= chance {
                    pool.push(dna.clone());
                }
            }
        }
        MatingPool::new(pool)
    }


    /// ## cross_over
    /// recieves a `MatingPool` that can be acuired from `get_mating_pool` method
    /// and returns the next generation of population
    pub fn cross_over(pool: MatingPool) -> Population{
        let mut pops: Vec<DNA> = vec![];
        for _i in 0..POP_SIZE {
            // get parent A 
            let parent_a = pool.get_rand();
            // get parent B
            let parent_b = pool.get_rand();
            // mating
            let mut child = parent_a.cross_over(parent_b);
            child.mutate();
            pops.push(child);
        }

        Population{ pops }
    }

    // pub fn next_generation(&self){
    //     // get mating pool
    //     // let pool = 
    // }

}
