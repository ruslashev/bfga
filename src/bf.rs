#[derive(Clone)]
pub enum BfErr {
    SyntaxError,
    InstrLimitExceeded,
    LogicError,
}

const TAPE_SIZE: usize = 1_000;

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

fn find_matching_brackets(src: &Vec<char>) -> Vec<usize> {
    let mut matching_brackets = vec![0; src.len()];
    let mut count = 0;
    let mut idx_per_count = vec![0; src.len()];

    for (idx, &instr) in src.iter().enumerate() {
        if instr == '[' {
            idx_per_count[count] = idx;
            count += 1;
        } else if instr == ']' {
            count -= 1;

            matching_brackets[idx_per_count[count]] = idx;
            matching_brackets[idx] = idx_per_count[count];
        }
    }

    matching_brackets
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

    let matching_brackets = find_matching_brackets(src);

    loop {
        if instr_ptr >= src.len() {
            break
        }

        if max_intructions != 0 {
            num_instructions += 1;
            if num_instructions > max_intructions {
                return Err(BfErr::InstrLimitExceeded)
            }
        }

        let instr: char = src[instr_ptr];

        instr_ptr = match instr {
            '+' => {
                tape[tape_ptr] = tape[tape_ptr].wrapping_add(1);
                instr_ptr + 1
            },
            '-' => {
                tape[tape_ptr] = tape[tape_ptr].wrapping_sub(1);
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
                    matching_brackets[instr_ptr]
                } else {
                    instr_ptr + 1
                },
            ']' =>
                if tape[tape_ptr] != 0 {
                    matching_brackets[instr_ptr]
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
fn hello_world_test() {
    let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
               ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    match interpret_brainfuck(&src.chars().collect(), 10_000) {
        Ok(output) => assert_eq!(output, "Hello World!\n"),
        Err(_)     => panic!("oopsie")
    }
}

#[test]
fn matching_brackets_test() {
    let src = ".[..[]..[].].";
    let expected = vec![0, 11, 0, 0, 5, 4, 0, 0, 9, 8, 0, 1, 0];

    assert_eq!(expected, find_matching_brackets(&src.chars().collect()));
}

