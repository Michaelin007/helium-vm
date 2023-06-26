pub mod instruction;
pub mod vm;
pub mod repl;
pub mod assembler;
extern  crate nom;

#[macro_use]

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
