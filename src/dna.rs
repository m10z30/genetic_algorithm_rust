use crate::random::AlphanumericAndSpace;
use rand::{thread_rng, Rng};

const MUTATION_RATE: i32 = 1;

#[derive(Clone, Debug)]
pub struct DNA {
    pub genes: String,
    pub fitness: f32
}


impl DNA {
    /// Create new DNA object.
    pub fn new(target: &String) -> DNA{
        DNA{ genes: DNA::random_genes(target), fitness: 0.0 }
    }

    /// generate a random string.
    pub fn random_genes(target: &String) -> String {
        thread_rng().sample_iter(AlphanumericAndSpace).take(target.len()).map(char::from).collect()
    }

    /// calculate fitness based on the assumption 
    /// that self.genes and target are the same length.
    pub fn calculate_fitness(&mut self, target: &String) {
        let mut score = 0;
        for i in 0..self.genes.len() {
            if target.chars().nth(i) == self.genes.chars().nth(i) {
                score  += 1 ;
            }
        }
        self.fitness = score as f32 / target.len() as f32;
    }

    /// cross over the genes of self and other based on 
    /// the assumption that both are the same length
    /// returns new DNA object.
    pub fn cross_over(&self, other: &DNA) -> DNA {
        let mut rng = thread_rng();
        let mut genes = String::from("");
        for i in 0..self.genes.len() {
            if rng.gen_range(1..=10) > 5 {
                genes.push(self.genes.chars().nth(i).expect("the length of genes do not match"));
            } else {
                genes.push(other.genes.chars().nth(i).expect("the length of genes do not match"));
            }
        }
        DNA { genes, fitness: 0.0 }
    }

    /// mutates self.genes by randomly changing characters.
    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.genes.len() {
            if rng.gen_range(0..100) < MUTATION_RATE {
                let new_char: String = thread_rng().sample_iter(AlphanumericAndSpace).take(1).map(char::from).collect();
                self.genes.replace_range(
                    self.genes
                        .char_indices()
                        .nth(i)
                        .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                        .unwrap(),
                    new_char.as_str()
                );
            }
        }        
    }


}

