use parser::Parse;

mod builtins;
mod error;
mod evaluate;
mod value;

pub fn parse(input: &str) -> Parse {
    parser::parse(input)
}
