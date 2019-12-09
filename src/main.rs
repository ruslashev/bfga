use rand::Rng;
mod bf;

const target: &str = "yummy cummy in my tummy";
const initial_population_size : u64 = 100;
const mutation_prob: f64 = 0.05;
const elitism_ratio : f64 = 1. / 10.;
const can_breed_ratio : f64 = 1. / 2.;

type Gene = char;

type Chromosome = Vec<Gene>;

#[derive(Clone)]
struct Individual {
    chromosome: Chromosome,
    fitness: u64
}

type Population = Vec<Individual>;

impl Individual {
    fn new(chr: Chromosome) -> Individual {
        Individual { fitness: fitness(&chr), chromosome: chr }
    }
}

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

    for _ in 0..target.len() {
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

fn mate_crossover(x: &Individual, y: &Individual) -> Individual {
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

    Individual::new(child_chr)
}

fn mate_random_gene(x: &Individual, y: &Individual) -> Individual {
    let mut child_chr: Chromosome = Vec::new();

    for i in 0..x.chromosome.len() {
        let p: f64 = rand_float!();
        let x_prob = (1. - mutation_prob) / 2.;
        let y_prob = 1. - mutation_prob;

        child_chr.push(
            if p < x_prob {
                x.chromosome[i]
            } else if p < y_prob {
                y.chromosome[i]
            } else {
                random_gene()
            })
    }

    Individual::new(child_chr)
}

fn mate(x: &Individual, y: &Individual) -> Individual {
    mate_random_gene(x, y)
}

fn select(population: &Population) -> Population {
    let mut new_generation: Population = Vec::new();
    let num_elite: usize = (population.len() as f64 * elitism_ratio).round() as usize;
    let num_rest = population.len() - num_elite;

    for i in 0..num_elite {
        new_generation.push(population[i].clone())
    }

    for _ in 0..num_rest {
        let p1_idx = rand_in_range!(0, ((population.len() - 1) as f64 * can_breed_ratio) as usize);
        let parent1: &Individual = &population[p1_idx];
        let mut p2_idx;

        loop {
            p2_idx = rand_in_range!(0, ((population.len() - 1) as f64 * can_breed_ratio) as usize);
            if p2_idx != p1_idx {
                break
            }
        }

        let parent2: &Individual = &population[p2_idx];

        new_generation.push(mate(parent1, parent2));
    }

    new_generation
}

fn main() {
    let mut generation = 0;
    let mut population: Population = Vec::new();

    for _ in 0..initial_population_size {
        population.push(Individual::new(random_chromosome()))
    }

    loop {
        population.sort_by(|x, y| x.fitness.cmp(&y.fitness));

        println!("generation: {:3} string: {} fitness: {}",
                 generation,
                 population[0].chromosome.iter().collect::<String>(),
                 population[0].fitness);

        if population[0].fitness == 0 {
            break
        }

        population = select(&population);

        generation += 1;
    }
}

