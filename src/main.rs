use genetic_algorithm::population::Population;


fn main() {
    // target phrase
    let target = String::from("to be or not be");
    
    // initialize population
    let mut population = Population::new(&target);

    // start at generation 0
    let mut g_count = 0;

    loop{
        // calculate fitness
        population.calculate_total_fitness(&target);    

        // get total fitness
        let total_fitness = population.total_fitness();
        // get best
        let best = population.find_best();
        println!("Generation {}", g_count);
        println!("Total Fitness: {}", total_fitness);
        println!("Best DNA: {}", best.genes);
        println!("Fitness: {}", best.fitness);

        if best.genes == target {
            break;
        }
        // get mating pool
        let pool = population.get_mating_pool();
        // cross over and get new population
        population = Population::cross_over(pool);
        // increament generation
        g_count += 1;
    }
}
