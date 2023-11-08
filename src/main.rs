mod evaluator;
mod expr;
mod parser;

pub use evaluator::eval;
pub use expr::Expr;
pub use parser::parse;

fn main() {
    // Test cases
    let test_cases = vec![
        ("(add 1 2)", 3),
        ("(mult 3 4)", 12),
        ("(add (mult 2 3) 4)", 10),
        ("(mult (add 1 2) (add 3 4))", 21),
    ];

    for (input, expected) in test_cases {
        match parse(input) {
            Ok(expr) => match eval(&expr) {
                Ok(result) => {
                    assert_eq!(result, expected);
                    println!("Test passed: {} = {}", input, result);
                }
                Err(e) => println!("Error evaluating {}: {}", input, e),
            },
            Err(e) => println!("Error parsing {}: {}", input, e),
        }
    }
}
