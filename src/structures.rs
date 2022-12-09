use genetic_algorithms::traits::{GeneT, GenotypeT};
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Default, PartialEq, Deserialize)]
pub struct Gene{
    pub id: i32,
    pub x: f64,
    pub y: f64
}
impl GeneT for Gene{
    fn new()->Gene{
        return Gene{id: -1, x: 0.0, y: 0.0};
    }
    fn get_id(&self) -> &i32{
        return &self.id;
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<Gene>{
    pub dna: Vec<Gene>,
    pub phenotype: f64,
}
impl GenotypeT<Gene> for Genotype<Gene>{
    fn get_dna(&self) -> &Vec<Gene> {
        &self.dna
    }
    fn get_dna_mut(&mut self) -> &mut Vec<Gene> {
        &mut self.dna
    }
    fn get_phenotype(&self) -> &f64 {
        return &self.phenotype;
    }
    fn calculate_phenotype(&mut self) {

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
        self.phenotype = distance;
    }
    fn new() -> Self {
       return Genotype{
        dna: Vec::new(),
        phenotype: 0.0,
       }
    }
}