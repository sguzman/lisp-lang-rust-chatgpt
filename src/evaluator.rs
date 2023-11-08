use crate::expr::Expr;

pub fn eval(expr: &Expr) -> Result<i32, String> {
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

// ... existing evaluator code ...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expr;

    #[test]
    fn test_eval_int() {
        let expr = Expr::Int(42);
        assert_eq!(eval(&expr), Ok(42));
    }

    #[test]
    fn test_eval_add_two_numbers() {
        let expr = Expr::Add(vec![Expr::Int(1), Expr::Int(2)]);
        assert_eq!(eval(&expr), Ok(3));
    }

    #[test]
    fn test_eval_add_multiple_numbers() {
        let expr = Expr::Add(vec![Expr::Int(1), Expr::Int(2), Expr::Int(3)]);
        assert_eq!(eval(&expr), Ok(6));
    }

    #[test]
    fn test_eval_add_no_numbers() {
        let expr = Expr::Add(Vec::new());
        assert_eq!(eval(&expr), Ok(0));
    }

    #[test]
    fn test_eval_mult_two_numbers() {
        let expr = Expr::Mult(vec![Expr::Int(3), Expr::Int(4)]);
        assert_eq!(eval(&expr), Ok(12));
    }

    #[test]
    fn test_eval_mult_multiple_numbers() {
        let expr = Expr::Mult(vec![Expr::Int(2), Expr::Int(3), Expr::Int(4)]);
        assert_eq!(eval(&expr), Ok(24));
    }

    #[test]
    fn test_eval_mult_no_numbers() {
        let expr = Expr::Mult(Vec::new());
        assert_eq!(eval(&expr), Ok(1));
    }

    #[test]
    fn test_eval_nested_expressions() {
        let expr = Expr::Add(vec![
            Expr::Int(1),
            Expr::Mult(vec![Expr::Int(2), Expr::Int(3)]),
        ]);
        assert_eq!(eval(&expr), Ok(7));
    }

    #[test]
    fn test_eval_deeply_nested_expressions() {
        let expr = Expr::Mult(vec![
            Expr::Add(vec![Expr::Int(1), Expr::Int(2)]),
            Expr::Add(vec![
                Expr::Int(3),
                Expr::Mult(vec![Expr::Int(4), Expr::Int(5)]),
            ]),
        ]);
        assert_eq!(eval(&expr), Ok(60));
    }

    #[test]
    fn test_eval_with_unexpected_expr() {
        let expr = Expr::Add(vec![
            Expr::Int(1),
            Expr::Mult(vec![Expr::Int(2), Expr::Add(vec![])]), // This should be an error
        ]);
        assert!(eval(&expr).is_err());
    }

    // ... You can add more tests to cover other cases as needed ...
}
