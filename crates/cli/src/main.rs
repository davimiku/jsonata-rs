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

        let parse = parse(trimmed_input);
        println!("{}", parse.debug_tree());

        let root = ast::Root::cast(parse.syntax()).unwrap();

        let var_defs = root
            .exprs()
            .filter_map(|expr| {
                if let ast::Expr::VariableDef(var_def) = expr {
                    Some(var_def.value())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        dbg!(var_defs);

        let (_, hir) = hir::lower(root);
        dbg!(hir);

        input.clear();
    }
}
