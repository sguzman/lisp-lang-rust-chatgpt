mod evaluator;
mod expr;
mod parser;

pub use evaluator::eval;
pub use expr::Expr;
pub use parser::parse;

fn main() {
    // The main function can be used to run the program, if applicable.
    // For now, it might just indicate that the tests can be run with `cargo test`.
    println!("Run `cargo test` to execute the test suite.");
}
