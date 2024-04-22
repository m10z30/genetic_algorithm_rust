use crate::dna::DNA;
use rand::prelude::*;

pub struct MatingPool {
    pops: Vec<DNA>,
}


impl MatingPool {
    /// Create new MatingPool
    pub fn new(pops: Vec<DNA>) -> MatingPool {
        MatingPool{ pops }
    }

    /// get random DNA from pops
    pub fn get_rand(&self) -> &DNA {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.pops.len());
        &self.pops[index]
    }
}
