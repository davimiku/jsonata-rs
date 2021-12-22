use core::parse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;
        let trimmed_input = input.trim_end();

        let parse = parse(&trimmed_input);
        println!("{}", parse.debug_tree());

        input.clear();
    }
}
