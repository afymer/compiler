use crate::keyword::Keyword;
use crate::literal::Literal;
use crate::operator::Operator;

pub enum Token {
    Literal(Literal),
    Keyword(Keyword),
    Operator(Operator),
    Symbol(String),
}