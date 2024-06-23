use std::fmt::{self, Debug, Formatter};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(lhs, op, rhs) => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
            Expr::Error => write!(f, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Opcode::*;
        match *self {
            Mul => write!(f, "*"), 
            Div => write!(f, "/"),
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
        }
    }
}
