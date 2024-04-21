# Genetic Algorithm Implementation Using Rust

this is a simple implementation of `Genetic Algorithm` using `rust`


## Concept

this is a recreation of [this](https://github.com/m10z30/genetic_algorithm_cpp) project I made using `c++`,
the idea is that we have a `target phrase` and we have a sample `population` that gets closer to the `target phrase` with each `generation`.


## Steps
the program goes through few steps:

1- Initialize New Population.  
2- calculate fitness of population.  
3- find `DNA` with best fitness.  
4- if best fitness is equal to `target phrase` stop the program.  
5- get mating pool.  
6- cross over population in mating pool and get new generation.  
7- repeat from step 2  

