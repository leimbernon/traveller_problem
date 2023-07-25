use std::{error::Error, process, time::Instant};
use genetic_algorithms::{population::Population, configuration::{GaConfiguration, ProblemSolving, LimitConfiguration, SelectionConfiguration}, operations::{Selection, Crossover, Mutation, Survivor}};
use rand::Rng;
use structures::Genotype;
use crate::structures::Gene;

mod structures;

//Function to read the CSV
fn csv_reader() -> Result<Vec<Gene>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("./data/cities_minimized.csv")?;
    let mut genes = Vec::new();
    for result in rdr.deserialize() {
        let record: Gene = result?;
        genes.push(record);
        println!("{:?}", record);
    }
    Ok(genes)
}

//Function to initialize the population
fn intialize_population(genes: Vec<Gene>, population_size: i32) -> Population<Gene, Genotype<Gene>>{

    let mut individuals = Vec::new();

    //Creates the individuals to fill the population
    for i in 0..population_size{

        println!("Initialization of the individual: {}", i);
        let mut rng = rand::thread_rng();
        let mut tmp_genes = genes.clone();
        let mut dna = Vec::new();

        //1- Selects the genes randomly from the vector without repeating them
        for _j in 0..tmp_genes.len(){
            let index = rng.gen_range(0..tmp_genes.len());
            let gene = tmp_genes.get(index).copied().unwrap();
            tmp_genes.remove(index);

            dna.push(gene);
        }

        //2- Sets the dna into the individual vector
        individuals.push(Genotype{dna, fitness:0.0, age:0});

    }

    //3- Sets the population
    return Population::new(individuals);
}


fn main() {
    let csv_read = csv_reader();
    if let Err(err) = csv_read {
        println!("error running example: {}", err);
        process::exit(1);
    }else{

        //We initialize the population and the configuration
        let population = intialize_population(csv_read.unwrap(), 100);
        let configuration = GaConfiguration{
            number_of_threads: Some(8),
            limit_configuration: LimitConfiguration{max_generations: 1000, fitness_target: None, problem_solving: ProblemSolving::Minimization},
            selection_configuration: Some(SelectionConfiguration{number_of_couples: 100}),
            crossover_configuration: None,
            selection: Selection::Tournament,
            crossover: Crossover::Cycle,
            mutation: Mutation::Swap,
            survivor: Survivor::Fitness,
        };

        //We run genetic algorithms
        let start = Instant::now();
        let best_population = genetic_algorithms::ga::run(population, configuration);
        let duration = start.elapsed();

        println!("Best fitness: {}", best_population.individuals[0].fitness);
        println!("Time elapsed in genetic algorithms() is: {:?}", duration);
    }
}
