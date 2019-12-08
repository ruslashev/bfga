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

macro_rules! rand_float {
    () => {
        rand::thread_rng().gen()
    }
}

fn random_gene() -> Gene {
    let valid_genes: String = String::from("abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890, .-;:_!\"#%&/()=?@${[]}");

    return valid_genes.as_bytes()[rand_in_range!(0, valid_genes.len() - 1)] as char;
}

fn random_chromosome() -> Chromosome {
    let mut chr: Chromosome = Vec::new();

    for _ in 1..target.len() {
        chr.push(random_gene())
    }

    chr
}

fn fitness(chr: &Chromosome) -> u64 {
    let mut fitness: u64 = 0;

    for (idx, target_gene) in target.chars().enumerate() {
        if chr[idx] != target_gene {
            fitness += 1;
        }
    }

    fitness
}

fn mate(x: &Individual, y: &Individual) -> Individual {
    let mut child_chr: Chromosome = Vec::new();
    let len = x.chromosome.len();
    let crossover = rand_in_range!(0, len - 1);

    for i in 0..crossover {
        let p: f64 = rand_float!();
        child_chr.push(if p <= mutation_prob { random_gene() } else { x.chromosome[i] })
    }

    for i in crossover..len {
        let p: f64 = rand_float!();
        child_chr.push(if p <= mutation_prob { random_gene() } else { y.chromosome[i] })
    }

    Individual { fitness: fitness(&child_chr), chromosome: child_chr }
}

fn main() {

}

