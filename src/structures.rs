use genetic_algorithms::traits::{GeneT, GenotypeT};
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Default, PartialEq, Deserialize)]
pub struct Gene{
    pub id: i32,
    pub x: f64,
    pub y: f64,
}
impl GeneT for Gene{
    fn get_id(&self) -> i32{
        return self.id;
    }
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype{
    pub dna: Vec<Gene>,
    pub fitness: f64,
    pub age: i32,
}
impl GenotypeT for Genotype{
    type Gene = Gene;
    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn set_dna(&mut self, dna: &[Self::Gene]){
        self.dna = dna.to_vec();
    }
    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
    fn set_age(&mut self, age:i32){
        self.age = age;
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn calculate_fitness(&mut self) {

        let mut distance = 0.0;
        let mut last_gene_x = 0.0;
        let mut last_gene_y = 0.0;
        let mut first_point = true;

        for gene in &self.dna {
            if first_point {
                last_gene_x = gene.x;
                last_gene_y = gene.y;
                first_point = false;
            }else{
                distance += ((gene.x - last_gene_x).powf(2.0) + (gene.y - last_gene_y).powf(2.0)).sqrt();
                last_gene_x = gene.x;
                last_gene_y = gene.y;
            }
        }
        self.fitness = distance;
    }
}