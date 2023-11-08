#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    Add(Vec<Expr>),
    Mult(Vec<Expr>),
}
