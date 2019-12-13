#[derive(Clone)]
pub enum BfErr {
    SyntaxError,
    InstrLimitExceeded,
    LogicError,
}

const TAPE_SIZE: usize = 30_000;

fn check_bf_syntax(src: &Vec<char>) -> bool {
    let mut balance = 0;

    for instr in src {
        balance +=
            match instr {
                '[' => 1,
                ']' => -1,
                _ => 0
            };
        if balance < 0 {
            return false
        }
    }

    balance == 0
}

fn find_matching_closing_bracket(instr_ptr: usize, src: &Vec<char>) -> usize {
    let mut balance = 0;

    for (idx, &instr) in src[(instr_ptr + 1)..].iter().enumerate() {
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

fn find_matching_opening_bracket(instr_ptr: usize, src: &Vec<char>) -> usize {
    let mut balance = 0;

    for (idx, &instr) in src[..instr_ptr].iter().rev().enumerate() {
        if instr == '[' {
            if balance == 0 {
                return instr_ptr - idx - 1
            } else {
                balance += 1
            }
        } else if instr == ']' {
            balance -= 1
        }
    }

    panic!("found no matching bracket but should");
}

pub fn interpret_brainfuck(src: &Vec<char>, max_intructions: u64) -> Result<String, BfErr> {
    let mut instr_ptr: usize = 0;
    let mut tape_ptr: usize = 0;
    let mut tape: [u8; TAPE_SIZE] = [0; TAPE_SIZE];
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

        let instr: char = src[instr_ptr] as char;

        instr_ptr = match instr {
            '+' => {
                if tape[tape_ptr] < 255 {
                    tape[tape_ptr] += 1;
                } else {
                    return Err(BfErr::LogicError);
                }
                instr_ptr + 1
            },
            '-' => {
                if tape[tape_ptr] > 0 {
                    tape[tape_ptr] -= 1;
                } else {
                    return Err(BfErr::LogicError);
                }
                instr_ptr + 1
            },
            '>' => {
                if tape_ptr < TAPE_SIZE - 1 {
                    tape_ptr += 1
                } else {
                    return Err(BfErr::LogicError);
                }
                instr_ptr + 1
            },
            '<' => {
                if tape_ptr > 0 {
                    tape_ptr -= 1
                } else {
                    return Err(BfErr::LogicError);
                }
                instr_ptr + 1
            },
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
            _   => instr_ptr + 1
        }
    }

    Ok(output)
}

#[test]
fn test() {
    let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
               ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    match interpret_brainfuck(&src.chars().collect(), 10_000) {
        Ok(output) => assert_eq!(output, "Hello World!\n"),
        Err(_)     => panic!("oopsie")
    }
}

