mod cli;
use cli::CLI;

//windows:
// ev:  rustup target add x86_64-pc-windows-gnu
// sudo apt update && sudo apt install mingw-w64
// cargo build --target x86_64-pc-windows-gnu
//

/*
    Gå igenom under presentation
    0. go through program
    1. commitents, hur det går med commitment 1 och 2, att jag "bit of more than i could chew"
    (2. UML - Maintainability  and clean code?)
    3. Questions
    4. Documentation and instructions.
*/

// frågor till kund:
// 2. what more environment block are exceptions to the indentationrule?
// 3. what LaTeX commands does the blank lines rule affect?

// ./latex-Linter
// ./latex-linter latex.txt
// ./latex-linter latex.tex // -> show first.tex file, result before and after
// ./latex-linter first.tex

fn main() {
    let mut cli = CLI::new();
    CLI::parse_args(&mut cli);
    CLI::run(&mut cli)
}
