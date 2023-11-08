use crate::expr::Expr;
use std::str::Chars;

// This function remains the same
pub fn parse(input: &str) -> Result<Expr, String> {
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
    skip_whitespace(chars);
    while chars.peek().map_or(false, |&ch| ch != ')') {
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

// ... existing parser code ...

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Peekable;
    use std::str::Chars;

    fn parse_helper(input: &str) -> Result<Expr, String> {
        let mut chars = input.chars().peekable();
        parse_expr(&mut chars)
    }

    #[test]
    fn test_parse_number_simple() {
        assert_eq!(parse_helper("42"), Ok(Expr::Int(42)));
    }

    #[test]
    fn test_parse_number_with_whitespace() {
        assert_eq!(parse_helper("  42 "), Ok(Expr::Int(42)));
    }

    #[test]
    fn test_parse_number_negative() {
        assert!(parse_helper("-42").is_err());
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(parse_helper("abc").is_err());
    }

    #[test]
    fn test_parse_add_valid() {
        assert_eq!(
            parse("(add 1 2)"),
            Ok(Expr::Add(vec![Expr::Int(1), Expr::Int(2)]))
        );
    }

    #[test]
    fn test_parse_add_extra_whitespace() {
        assert_eq!(
            parse("(add  1   2  )"),
            Ok(Expr::Add(vec![Expr::Int(1), Expr::Int(2)]))
        );
    }

    #[test]
    fn test_parse_add_unbalanced_parentheses() {
        assert!(parse("(add 1 2").is_err());
    }

    #[test]
    fn test_parse_add_no_arguments_returns_identity() {
        assert_eq!(parse("(add)"), Ok(Expr::Add(Vec::new())));
    }

    #[test]
    fn test_parse_mult_no_arguments_returns_identity() {
        assert_eq!(parse("(mult)"), Ok(Expr::Mult(Vec::new())));
    }

    #[test]
    fn test_parse_mult_valid() {
        assert_eq!(
            parse("(mult 3 4)"),
            Ok(Expr::Mult(vec![Expr::Int(3), Expr::Int(4)]))
        );
    }

    #[test]
    fn test_parse_mult_nested_expressions() {
        assert_eq!(
            parse("(mult (add 1 2) 3)"),
            Ok(Expr::Mult(vec![
                Expr::Add(vec![Expr::Int(1), Expr::Int(2)]),
                Expr::Int(3)
            ]))
        );
    }

    #[test]
    fn test_parse_mult_invalid_keyword() {
        assert!(parse("(multiply 3 4)").is_err());
    }

    // ... You can add more tests to cover other cases as needed ...
}
