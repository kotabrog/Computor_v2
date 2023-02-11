use std::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
    MatrixMul,
    Pow,
    Paren,
    RParen,
}

impl Operator {
    /// rhs has higher priority than self
    pub fn priority(&self, rhs: &Self) -> bool {
        match *self {
            Self::Plus | Self::Minus => {
                match rhs {
                    Self::Mul | Self::Div | Self::Rem | Self::MatrixMul | Self::Pow => true,
                    _ => false
                }
            },
            Self::Mul | Self::Div | Self::Rem | Self::MatrixMul => {
                match rhs {
                    Self::Pow => true,
                    _ => false
                }
            },
            Self::Pow => {
                match rhs {
                    Self::Paren => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
}


impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Rem => write!(f, "%"),
            Operator::Pow => write!(f, "^"),
            Operator::MatrixMul => write!(f, "**"),
            Operator::Paren => write!(f, "("),
            Operator::RParen => write!(f, ")"),
        }
    }
}
