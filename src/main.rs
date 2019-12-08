const INSTR_LIMIT: u64 = 10_000;

enum BfErr {
    SyntaxError,
    InstrLimitExceeded,
}

fn interpret_brainfuck(src: &String, max_intructions: u64) -> Result<String, BfErr> {
    let mut num_instructions = 0;
    let mut tape_ptr: usize = 0;
    let mut tape: [u8; 1000] = [0; 1000];

    fn check_syntax(src: &String) -> bool {
        let mut balance = 0;

        for instr in src.chars() {
            balance +=
                match instr {
                    '[' => -1,
                    ']' => 1,
                    _ => 0
                }
        }

        balance == 0
    }

    if check_syntax(src) == false {
        return Err(BfErr::SyntaxError)
    }

    for instr in src.chars() {
        num_instructions += 1;
        if num_instructions > max_intructions {
            return Err(BfErr::InstrLimitExceeded)
        }

        println!("token: {}", instr);

        match instr {
            '+' => tape[tape_ptr] += 1,
            '-' => tape[tape_ptr] -= 1,
            '>' => tape_ptr += 1,
            '<' => tape_ptr -= 1,
            _   => return Err(BfErr::SyntaxError)
        }
    }

    Err(BfErr::SyntaxError)
}

fn main() {
    let example_src = String::from("+++++++++++++++++++++++++++++++++.");

    interpret_brainfuck(&example_src, INSTR_LIMIT);

    println!("Hello, world!");
}

