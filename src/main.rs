use ctrlc;
use std::sync::{atomic, Arc};
mod bf;
mod rand;

const INITIAL_POPULATION_SIZE: u64 = 1000;
const MUTATION_PROB_PERC: u64 = 10;
const ELITISM_RATIO: f64 = 5. / 100.;
const INITIAL_PROGRAM_LENGTH: usize = 160;
const INSTR_LIMIT: u64 = 100_000;
const BAD_PROGRAM_PENALTY: u64 = 10000;
const TOURNAMENT_SIZE: u64 = 2;
const REMOVE_MUT_PROB: u64 = 20;
const INSERT_MUT_PROB: u64 = 10;
const MODIFY_MUT_PROB: u64 = 70;

static TARGET: &str = "hello";
static VALID_GENES: &str = "+++++++-------<<>>.[]";

type Rng = rand::Wyhash64RNG;

type Gene = char;

type Chromosome = Vec<Gene>;

#[derive(Clone)]
struct Individual {
    chromosome: Chromosome,
    bf_result: bf::BfResult,
    fitness: u64,
}

type Population = Vec<Individual>;

impl Individual {
    fn new(chromosome: Chromosome) -> Individual {
        let bf_result = bf::interpret_brainfuck(&chromosome, INSTR_LIMIT);
        let fitness = fitness(&chromosome, &bf_result);

        Individual { chromosome, bf_result, fitness }
    }
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.bf_result {
            Ok((output, _)) => write!(f, "output: \"{}\"", output),
            Err(err)        => write!(f, "{}", err),
        }
    }
}

fn random_gene(rng: &mut Rng) -> Gene {
    return VALID_GENES.as_bytes()[rng.gen_in_size(VALID_GENES.len())] as char;
}

fn random_chromosome(rng: &mut Rng) -> Chromosome {
    let mut chr: Chromosome = Vec::new();

    for _ in 0..INITIAL_PROGRAM_LENGTH {
        chr.push(random_gene(rng))
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

    let (smaller, larger): (&str, &str) = if x.len() < y.len() { (x, y) } else { (y, x) };

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

fn fitness(chromosome: &Chromosome, bf_result: &bf::BfResult) -> u64 {
    let fitness = match bf_result {
        Ok((output, num_instructions)) =>
            string_difference(&output, TARGET) * 1 +
            program_length(chromosome) * 0 +
            num_instructions * 0,
        Err(_) => BAD_PROGRAM_PENALTY
    };

    fitness
}

fn mutate(rng: &mut Rng, chr: &mut Chromosome)  {
    for i in 0..chr.len() {
        let p = rng.gen_percent();
        if p > MUTATION_PROB_PERC {
            continue;
        }

        let mutation = rng.gen_percent();

        if mutation <= REMOVE_MUT_PROB {
            chr[i] = ' ';
        } else if mutation <= REMOVE_MUT_PROB + INSERT_MUT_PROB {
            chr.insert(i, random_gene(rng));
        } else if mutation <= REMOVE_MUT_PROB + INSERT_MUT_PROB + MODIFY_MUT_PROB {
            chr[i] = random_gene(rng);
        }
    }
}

fn mate(rng: &mut Rng, x: &Individual, y: &Individual) -> (Individual, Individual) {
    let (mut child1_chr, mut child2_chr) = (Vec::new(), Vec::new());
    let (smaller, larger) =
        if x.chromosome.len() < y.chromosome.len() {
            (&x.chromosome, &y.chromosome)
        } else {
            (&y.chromosome, &x.chromosome)
        };
    let crossover = rng.gen_in_size(smaller.len());

    for i in 0..crossover {
        child1_chr.push(smaller[i]);
        child2_chr.push(larger[i]);
    }

    for i in crossover..smaller.len() {
        child1_chr.push(larger[i]);
        child2_chr.push(smaller[i]);
    }

    for i in smaller.len()..larger.len() {
        child1_chr.push(larger[i]);
    }

    mutate(rng, &mut child1_chr);
    mutate(rng, &mut child2_chr);

    (Individual::new(child1_chr), Individual::new(child2_chr))
}

fn tournament_select(rng: &mut Rng, population: &Population, k: u64) -> usize {
    let mut best = rng.gen_in_size(population.len());

    for _ in 0..k - 1 {
        let candidate = rng.gen_in_size(population.len());

        if population[candidate].fitness < population[best].fitness {
            best = candidate;
        }
    }

    best
}

fn select(rng: &mut Rng, population: &Population) -> Population {
    let mut new_generation: Population = Vec::new();
    let num_elite: usize = (population.len() as f64 * ELITISM_RATIO).round() as usize;
    let num_rest = population.len() - num_elite;

    for i in 0..num_elite {
        new_generation.push(population[i].clone())
    }

    for _ in 0..num_rest / 2 {
        let parent1 = &population[tournament_select(rng, population, TOURNAMENT_SIZE)];
        let parent2 = &population[tournament_select(rng, population, TOURNAMENT_SIZE)];

        let (child1, child2) = mate(rng, parent1, parent2);

        new_generation.push(child1);
        new_generation.push(child2);
    }

    new_generation
}

fn format_source(chromosome: &Chromosome) -> String {
    let mut chr_str = chromosome.iter().collect::<String>();
    chr_str.retain(|gene| gene != ' ');
    chr_str
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 80 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

fn install_ctrl_c_handler() -> Arc<atomic::AtomicBool> {
    let running = Arc::new(atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    running
}

fn main() {
    let running = install_ctrl_c_handler();

    let mut rng = rand::Wyhash64RNG::new();

    let mut generation = 0;
    let mut population: Population = Vec::new();

    for _ in 0..INITIAL_POPULATION_SIZE {
        population.push(Individual::new(random_chromosome(&mut rng)))
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
            break
        }

        population = select(&mut rng, &population);

        generation += 1;
    }

    println!("Source:\n{}", format_source(&population[0].chromosome));
}

