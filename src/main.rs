use ctrlc;
use random_fast_rng::Random;
use std::sync::{atomic, Arc};
mod bf;

const TARGET: &str = "hello";
const INITIAL_POPULATION_SIZE: u64 = 1000;
const MUTATION_PROB: f64 = 0.07;
const ELITISM_RATIO: f64 = 5. / 100.;
const CAN_BREED_RATIO: f64 = 2. / 3.;
const MATE_METHOD_CROSSOVER: bool = true;
const INITIAL_PROGRAM_LENGTH: usize = 400;
const INSTR_LIMIT: u64 = 100_000;
const BAD_PROGRAM_PENALTY: u64 = 1000;

type BfResult = Result<String, bf::BfErr>;

type Gene = char;

type Chromosome = Vec<Gene>;

#[derive(Clone)]
struct Individual {
    chromosome: Chromosome,
    bf_result: BfResult,
    fitness: u64,
}

type Population = Vec<Individual>;

impl Individual {
    fn new(chromosome: Chromosome) -> Individual {
        let source = chromosome.iter().collect::<String>();
        let bf_result = bf::interpret_brainfuck(&source, INSTR_LIMIT);
        let fitness = fitness(&chromosome, &bf_result);

        Individual { chromosome, bf_result, fitness }
    }
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.bf_result {
            Ok(output)                         => write!(f, "output: \"{}\"", output),
            Err(bf::BfErr::SyntaxError)        => write!(f, "syntax error"),
            Err(bf::BfErr::InstrLimitExceeded) => write!(f, "instruction limit exceeded"),
            Err(bf::BfErr::LogicError)         => write!(f, "logic error"),
        }
    }
}

macro_rules! rand_in_range {
    ($min:expr, $max:expr) => {
        $min + random_fast_rng::local_rng().get_usize() % ($max - $min + 1)
    }
}

macro_rules! rand_float {
    () => {
        random_fast_rng::local_rng().gen()
    }
}

fn random_gene() -> Gene {
    let valid_genes: String = String::from("++++----<<>>[].   ");

    return valid_genes.as_bytes()[rand_in_range!(0, valid_genes.len() - 1)] as char;
}

fn random_chromosome() -> Chromosome {
    let mut chr: Chromosome = Vec::new();

    for _ in 0..INITIAL_PROGRAM_LENGTH {
        chr.push(random_gene())
    }

    chr
}

fn string_difference(x: &str, y: &str) -> u64 {
    let mut difference: u64 = 0;

    if x == y {
        return 0
    }

    if x.len() == y.len() {
        for i in 0..x.len() {
            difference += (x.as_bytes()[i] as i16 - y.as_bytes()[i] as i16).abs() as u64
        }
        return difference
    }

    let (smaller,larger): (&str, &str) = if x.len() < y.len() { (x, y) } else { (y, x) };

    for i in 0..smaller.len() {
        difference += (smaller.as_bytes()[i] as i16 - larger.as_bytes()[i] as i16).abs() as u64
    }

    for i in smaller.len()..larger.len() {
        difference += larger.as_bytes()[i] as u64
    }

    difference
}

#[test]
fn diff_test() {
    assert_eq!(string_difference("aaa", "bbb"), 3);
    assert_eq!(string_difference("ccc", "bbb"), 3);

    assert_eq!(string_difference("aa", "bbb"), 2 + 'b' as u64);
    assert_eq!(string_difference("bbb", "aa"), 2 + 'b' as u64);

    assert_eq!(string_difference("aa", "aaa"), 'a' as u64);
    assert_eq!(string_difference("aaa", "aa"), 'a' as u64);
}

fn program_length(chromosome: &Chromosome) -> u64 {
    chromosome.iter().fold(0, |sum, &x| sum + if x == ' ' { 0 } else { 1 })
}

fn fitness(chromosome: &Chromosome, bf_result: &BfResult) -> u64 {
    let fitness = match bf_result {
        Ok(output) => string_difference(&output, TARGET) + program_length(chromosome) * 0,
        Err(_) => BAD_PROGRAM_PENALTY
    };

    fitness
}

fn mate_crossover(x: &Individual, y: &Individual) -> Individual {
    let mut child_chr: Chromosome = Vec::new();
    let len = x.chromosome.len();
    let crossover = rand_in_range!(0, len - 1);

    for i in 0..crossover {
        let p: f64 = rand_float!();
        child_chr.push(if p <= MUTATION_PROB { random_gene() } else { x.chromosome[i] })
    }

    for i in crossover..len {
        let p: f64 = rand_float!();
        child_chr.push(if p <= MUTATION_PROB { random_gene() } else { y.chromosome[i] })
    }

    Individual::new(child_chr)
}

fn mate_random_gene(x: &Individual, y: &Individual) -> Individual {
    let mut child_chr: Chromosome = Vec::new();

    for i in 0..x.chromosome.len() {
        let p: f64 = rand_float!();
        let x_prob = (1. - MUTATION_PROB) / 2.;
        let y_prob = 1. - MUTATION_PROB;

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
    if MATE_METHOD_CROSSOVER {
        mate_crossover(x, y)
    } else {
        mate_random_gene(x, y)
    }
}

fn select(population: &Population) -> Population {
    let mut new_generation: Population = Vec::new();
    let num_elite: usize = (population.len() as f64 * ELITISM_RATIO).round() as usize;
    let num_rest = population.len() - num_elite;

    for i in 0..num_elite {
        new_generation.push(population[i].clone())
    }

    for _ in 0..num_rest {
        let p1_idx = rand_in_range!(0, ((population.len() - 1) as f64 * CAN_BREED_RATIO) as usize);
        let parent1: &Individual = &population[p1_idx];
        let mut p2_idx;

        loop {
            p2_idx = rand_in_range!(0, ((population.len() - 1) as f64 * CAN_BREED_RATIO) as usize);
            if p2_idx != p1_idx {
                break
            }
        }

        let parent2: &Individual = &population[p2_idx];

        new_generation.push(mate(parent1, parent2));
    }

    new_generation
}

fn format_source(chromosome: &Chromosome) -> String {
    let mut chr_str = chromosome.iter().collect::<String>();
    chr_str.retain(|gene| gene != ' ');
    chr_str.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 80 == 0 {
                Some('\n')
            } else {
                None
            }.into_iter().chain(std::iter::once(c))
        })
    .collect::<String>()
}

fn install_ctrl_c_handler() -> Arc<atomic::AtomicBool> {
    let running  = Arc::new(atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || { r.store(false, atomic::Ordering::SeqCst); })
        .expect("Error setting Ctrl-C handler");
    running
}

fn main() {
    let running = install_ctrl_c_handler();

    let mut generation = 0;
    let mut population: Population = Vec::new();

    for _ in 0..INITIAL_POPULATION_SIZE {
        population.push(Individual::new(random_chromosome()))
    }

    loop {
        population.sort_by(|x, y| x.fitness.cmp(&y.fitness));

        println!("generation: {:5} fitness: {}, status: {}",
                 generation,
                 population[0].fitness,
                 population[0]);

        if population[0].fitness == 0 {
            break
        }

        if running.load(atomic::Ordering::SeqCst) == false {
            println!("Received interrupt. Exiting...");
            println!("Source:\n{}", format_source(&population[0].chromosome));
            break
        }

        population = select(&population);

        generation += 1;
    }
}

