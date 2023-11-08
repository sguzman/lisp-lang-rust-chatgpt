use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Expr {
    Int(i32),
    Add(Vec<Expr>),
    Mult(Vec<Expr>),
}

fn parse(input: &str) -> Result<Expr, String> {
    let mut chars = input.chars().peekable();
    parse_expr(&mut chars)
}

fn parse_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, String> {
    skip_whitespace(chars);
    if chars.peek() == Some(&'(') {
        chars.next(); // consume '('
        skip_whitespace(chars);
        let expr = match chars.next() {
            Some('a') => {
                chars.next(); // consume 'd'
                chars.next(); // consume 'd'
                parse_list(chars).map(Expr::Add)
            }
            Some('m') => {
                chars.next(); // consume 'u'
                chars.next(); // consume 'l'
                chars.next(); // consume 't'
                parse_list(chars).map(Expr::Mult)
            }
            _ => return Err("Expected 'add' or 'mult'".to_string()),
        };
        skip_whitespace(chars);
        if chars.next() != Some(')') {
            return Err("Expected ')'".to_string());
        }
        expr
    } else {
        parse_number(chars)
    }
}

fn parse_list(chars: &mut std::iter::Peekable<Chars>) -> Result<Vec<Expr>, String> {
    let mut list = Vec::new();
    while chars.peek() != Some(&')') {
        list.push(parse_expr(chars)?);
        skip_whitespace(chars);
    }
    Ok(list)
}

fn parse_number(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, String> {
    let mut number = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_digit(10) {
            number.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    if number.is_empty() {
        Err("Expected a number".to_string())
    } else {
        number
            .parse::<i32>()
            .map(Expr::Int)
            .map_err(|_| "Invalid number".to_string())
    }
}

fn skip_whitespace(chars: &mut std::iter::Peekable<Chars>) {
    while chars.peek().map_or(false, |ch| ch.is_whitespace()) {
        chars.next();
    }
}

fn eval(expr: &Expr) -> Result<i32, String> {
    match expr {
        Expr::Int(value) => Ok(*value),
        Expr::Add(args) => eval_add(args),
        Expr::Mult(args) => eval_mult(args),
    }
}

fn eval_add(args: &[Expr]) -> Result<i32, String> {
    let mut iter = args.iter();
    match iter.next() {
        Some(first_expr) => {
            let first_val = eval(first_expr)?;
            iter.try_fold(first_val, |acc, arg| Ok(acc + eval(arg)?))
        }
        None => Ok(0), // Adding nothing results in 0
    }
}

fn eval_mult(args: &[Expr]) -> Result<i32, String> {
    let mut iter = args.iter();
    match iter.next() {
        Some(first_expr) => {
            let first_val = eval(first_expr)?;
            iter.try_fold(first_val, |acc, arg| Ok(acc * eval(arg)?))
        }
        None => Ok(1), // Multiplying nothing results in 1
    }
}

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
