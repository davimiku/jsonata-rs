use parser::Parse;

mod builtins;
mod context;
mod evaluate;
mod value;

pub fn parse(input: &str) -> Parse {
    parser::parse(input)
}
