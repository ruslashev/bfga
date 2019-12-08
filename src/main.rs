use rand::Rng;
mod bf;

const target: &str = "yummy cummy in my tummy";
const initial_population_size : u64 = 100;
const mutation_prob: f64 = 0.05;
const elitism_ratio : f64 = 1. / 10.;
const can_breed_ratio : f64 = 1. / 2.;

type Gene = char;

type Chromosome = Vec<Gene>;

struct Individual {
    chromosome: Chromosome,
    fitness: u64
}

type Population = Vec<Individual>;

macro_rules! rand_in_range {
    ($min:expr, $max:expr) => {
        rand::thread_rng().gen_range($min, $max)
    }
}

fn random_gene() -> Gene {
    let valid_genes: String = String::from("abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890, .-;:_!\"#%&/()=?@${[]}");

    return valid_genes.as_bytes()[rand_in_range!(0, valid_genes.len() - 1)] as char;
}

fn main() {

}

