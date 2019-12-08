const INSTR_LIMIT: u64 = 10_000;

enum BfErr {
    SyntaxError,
    InstrLimitExceeded,
}

fn check_bf_syntax(src: &String) -> bool {
    let mut balance = 0;

    for instr in src.chars() {
        balance +=
            match instr {
                '[' => 1,
                ']' => -1,
                _ => 0
            }
    }

    balance == 0
}

fn find_matching_closing_bracket(instr_ptr: usize, src: &String) -> usize {
    let mut balance = 0;

    for (idx, instr) in src[instr_ptr..].chars().enumerate() {
        if instr == ']' {
            if balance == 0 {
                return idx
            } else {
                balance -= 1
            }
        } else if instr == '[' {
            balance += 1
        }
    }

    panic!("found no matching bracket but should");
}

fn find_matching_opening_bracket(instr_ptr: usize, src: &String) -> usize {
    let mut balance = 0;

    for (idx, instr) in src[..instr_ptr].chars().rev().enumerate() {
        if instr == '[' {
            if balance == 0 {
                return idx
            } else {
                balance += 1
            }
        } else if instr == ']' {
            balance -= 1
        }
    }

    panic!("found no matching bracket but should");
}

fn interpret_brainfuck(src: &String, max_intructions: u64) -> Result<String, BfErr> {
    let mut instr_ptr: usize = 0;
    let mut tape_ptr: usize = 0;
    let mut tape: [u8; 1000] = [0; 1000];
    let mut num_instructions = 0;
    let mut output = String::from("");

    if check_bf_syntax(src) == false {
        return Err(BfErr::SyntaxError)
    }

    loop {
        if instr_ptr >= src.len() {
            break
        }

        num_instructions += 1;
        if num_instructions > max_intructions {
            return Err(BfErr::InstrLimitExceeded)
        }

        let instr: char = src.as_bytes()[instr_ptr] as char;

        println!("{}: {}", instr_ptr, instr);

        instr_ptr = match instr {
            '+' => { tape[tape_ptr] += 1; instr_ptr + 1 },
            '-' => { tape[tape_ptr] -= 1; instr_ptr + 1 },
            '>' => { tape_ptr += 1;       instr_ptr + 1 },
            '<' => { tape_ptr -= 1;       instr_ptr + 1 },
            '[' =>
                if tape[tape_ptr] == 0 {
                    find_matching_closing_bracket(instr_ptr, src)
                } else {
                    instr_ptr + 1
                },
            ']' =>
                if tape[tape_ptr] != 0 {
                    find_matching_opening_bracket(instr_ptr, src)
                } else {
                    instr_ptr + 1
                },
            '.' => { output.push(tape[tape_ptr] as char); instr_ptr + 1 },
            _   => return Err(BfErr::SyntaxError)
        }
    }

    Ok(output)
}

fn main() {
    let example_src = String::from("+++++++++++++++++++++++++++++++++.");

    let res = interpret_brainfuck(&example_src, INSTR_LIMIT);
    match res {
        Ok(output)                     => println!("{}", output),
        Err(BfErr::SyntaxError)        => println!("syntax error"),
        Err(BfErr::InstrLimitExceeded) => println!("instr limit exceeded"),
    }
}

