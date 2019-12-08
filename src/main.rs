const INSTR_LIMIT: u64 = 100_000;

enum BfErr {
    SyntaxError,
    InstrLimitExceeded,
}

fn interpret_brainfuck(src: &String, max_intructions: u64) -> Result<String, BfErr> {
    let mut num_instructions = 0;
    let tape: [u8; 1000];

    for instr in src.chars() {
        num_instructions += 1;
        println!("token: {}", instr);
    }

    Err(BfErr::SyntaxError)
}

fn main() {
    let example_src = String::from("+++++++++++++++++++++++++++++++++.");

    interpret_brainfuck(&example_src, INSTR_LIMIT);

    println!("Hello, world!");
}

