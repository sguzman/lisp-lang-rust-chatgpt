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
