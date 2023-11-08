use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Expr {
    Int(i32),
    Add(Vec<Expr>),
    Mult(Vec<Expr>),
}

// This function remains the same
fn parse(input: &str) -> Result<Expr, String> {
    let mut chars = input.chars().peekable();
    parse_expr(&mut chars)
}

// Refactor parse_expr to handle the opening and closing of expressions
fn parse_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, String> {
    skip_whitespace(chars);
    if chars.peek() == Some(&'(') {
        chars.next(); // consume '('
        let expr = parse_inner_expr(chars)?;
        skip_whitespace(chars);
        if chars.next() != Some(')') {
            return Err("Expected ')'".to_string());
        }
        Ok(expr)
    } else {
        parse_number(chars)
    }
}

// New function to handle the inner part of an expression
fn parse_inner_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, String> {
    skip_whitespace(chars);
    match chars.next() {
        Some('a') => {
            expect_keyword(chars, "dd")?;
            parse_list(chars).map(Expr::Add)
        }
        Some('m') => {
            expect_keyword(chars, "ult")?;
            parse_list(chars).map(Expr::Mult)
        }
        _ => Err("Expected 'add' or 'mult'".to_string()),
    }
}

// New function to check for specific keywords
fn expect_keyword(chars: &mut std::iter::Peekable<Chars>, keyword: &str) -> Result<(), String> {
    for expected_char in keyword.chars() {
        match chars.next() {
            Some(ch) if ch == expected_char => continue,
            _ => return Err(format!("Expected '{}'", keyword)),
        }
    }
    Ok(())
}

// parse_list and parse_number can remain mostly unchanged
// ...

// The eval function and its helpers can remain unchanged for now
// ...

// The skip_whitespace function remains the same
// ...

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
