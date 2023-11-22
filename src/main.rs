use std::{error::Error, process, time::Instant};
use genetic_algorithms::{population::Population, operations::{Selection, Crossover, Mutation, Survivor}, configuration::{ProblemSolving, LogLevel}, ga::Ga, traits::ConfigurationT};
use rand::Rng;
use structures::Genotype;
use plotly::{Plot, Scatter, Layout, layout::Axis, common::Title};
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

//Function to write a plot with the results of the ga
fn write_plot(best_populations: Vec<Population<Genotype>>, names: Vec<String>){

    let mut plot = Plot::new();
    let layout = Layout::new().x_axis(Axis::new().title(Title::from("Generation number")))
                            .y_axis(Axis::new().title(Title::from("Distance")))
                            .title(Title::from("Comparison GA vs AGA in Traveller Problem"));

    for (i, population) in best_populations.iter().enumerate() {
        //We create the vectors needed for the plot
        let mut x = vec![];
        let mut y = vec![];

        for i in 0..population.individuals.len(){
            x.push(i);
            y.push(population.individuals[i].fitness);
        }

        let trace = Scatter::new(x, y).name(&names[i]);
        plot.add_trace(trace);
        plot.set_layout(layout.clone());
    }
    
    plot.write_html("out.html");
}

//Function to initialize the population
fn intialize_population(genes: Vec<Gene>, population_size: i32) -> Population<Genotype>{

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

        //We get the alleles
        let alleles = csv_read.unwrap();

        //We run genetic algorithms without adaptive genetic algorithms
        let start = Instant::now();
        let best_population_without_aga: Population<Genotype> = Ga::new()
            .with_threads(8)
            .with_logs(LogLevel::Info)
            .with_max_generations(5000)
            .with_problem_solving(ProblemSolving::Minimization)
            .with_best_individual_by_generation(true)
            .with_number_of_couples(10)
            .with_selection_method(Selection::Tournament)
            .with_crossover_method(Crossover::Cycle)
            .with_mutation_method(Mutation::Swap)
            .with_survivor_method(Survivor::Fitness)
            .with_alleles(alleles.clone())
            .with_genes_per_individual(alleles.len() as i32)
            .with_alleles_can_be_repeated(false)
            .with_population_size(100)
            .run();          
        let duration = start.elapsed();

        //We run genetic algorithms with adaptive genetic algorithms
        let start = Instant::now();
        let best_population_with_aga: Population<Genotype> = Ga::new()
            .with_threads(8)
            .with_logs(LogLevel::Info)
            .with_max_generations(5000)
            .with_problem_solving(ProblemSolving::Minimization)
            .with_best_individual_by_generation(true)
            .with_number_of_couples(10)
            .with_selection_method(Selection::Tournament)
            .with_crossover_method(Crossover::Cycle)
            .with_mutation_method(Mutation::Swap)
            .with_survivor_method(Survivor::Fitness)
            .with_alleles(alleles.clone())
            .with_genes_per_individual(alleles.len() as i32)
            .with_alleles_can_be_repeated(false)
            .with_population_size(100)
            .with_adaptive_ga(true)
            .with_crossover_probability_min(0.3)
            .with_crossover_probability_max(1.0)
            .with_mutation_probability_min(0.3)
            .with_mutation_probability_max(1.0)
            .run();          
        let aga_duration = start.elapsed();


        //Write the results
        println!("Best fitness: {}", best_population_without_aga.individuals[0].fitness);
        println!("Time elapsed in genetic algorithms() is: {:?}", duration);

        println!("Best aga fitness: {}", best_population_with_aga.individuals[0].fitness);
        println!("Time elapsed in adaptive genetic algorithms() is: {:?}", aga_duration);

        //We write the plot with the results
        write_plot(vec![best_population_without_aga, best_population_with_aga], vec![String::from("GA"), String::from("AGA")]);
    }
}
