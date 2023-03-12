#[derive(Debug, PartialEq)]
pub enum Expr {
    ENum(f32),
    EBinop(Box<Expr>, Binop, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Binop {
    EAdd,
    ESub,
    EMul,
    EDiv,
    EExp,
}
