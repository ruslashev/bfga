mod bf;

const INSTR_LIMIT: u64 = 10_000;

fn main() {
    let example_src = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");

    let res = bf::interpret_brainfuck(&example_src, INSTR_LIMIT);
    match res {
        Ok(output)                     => println!("{}", output),
        Err(bf::BfErr::SyntaxError)        => println!("syntax error"),
        Err(bf::BfErr::InstrLimitExceeded) => println!("instr limit exceeded"),
    }
}

