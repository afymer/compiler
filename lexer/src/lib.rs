#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::else_if_without_else,
    clippy::module_name_repetitions,
    reason = "bad lint"
)]
#![expect(
    clippy::single_call_fn,
    clippy::mod_module_file_paths,
    clippy::pub_with_shorthand,
    clippy::pattern_type_mismatch,
    reason = "style"
)]
#![expect(
    clippy::while_let_on_iterator,
    reason = "better to understand when the iterator is used after the loop breaks"
)]
#![expect(clippy::doc_include_without_cfg, reason = "see issue #13918")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "I want them all")]
#![expect(clippy::multiple_inherent_impl, reason = "useful when lots of methods")]

mod keyword;
mod literal;
mod location;
mod operator;
mod symbol;
mod token;

use core::str::Lines;
use std::path::Path;

use literal::Literal;
use location::Location;
use operator::Operator;
use token::Token;

/// Represents the location of a token, to allow clear error messages
#[derive(Default)]
pub struct TokenSpan<'filepath> {
    pub filepath: Option<&'filepath Path>,
    /// first character of the span
    pub start: Location,
    /// last character of the span
    pub end: Location,
}

impl<'filepath, 'b: 'filepath> From<(&'filepath Path, &'b Location)> for TokenSpan<'filepath> {
    fn from(value: (&'filepath Path, &'b Location)) -> Self {
        Self { filepath: Some(value.0), start: value.1.clone(), end: value.1.clone() }
    }
}

/// LocalizedToken is a Token with localisation information
pub struct LToken<'filepath> {
    /// token span that covers the token
    pub span: TokenSpan<'filepath>,
    /// actual token
    pub token: Token,
}

/// convenient type for a token stream
pub type Tokens<'filepath> = Vec<LToken<'filepath>>;

enum OperatorBuilder {
    None,
    One(char),
    Two(char, char),
    Three(char, char, char),
}

enum FlushCounter {
    One,
    Two,
    Three,
}

impl OperatorBuilder {
    fn push(&mut self, ch: char) -> Option<Operator> {
        let mut lexed_operator = None;
        *self = match self {
            Self::None => Self::One(ch),
            Self::One(first) => Self::Two(*first, ch),
            Self::Two(first, second) => Self::Three(*first, *second, ch),
            Self::Three(first, second, third) => {
                let (size, operator) = match (*first, *second, *third) {
                    ('<', '<', '=') => (FlushCounter::Three, Operator::ShiftLeftAssign),
                    ('>', '>', '=') => (FlushCounter::Three, Operator::ShiftRightAssign),
                    ('-', '>', _) => (FlushCounter::Two, Operator::Arrow),
                    ('+', '+', _) => (FlushCounter::Two, Operator::Increment),
                    ('-', '-', _) => (FlushCounter::Two, Operator::Decrement),
                    ('<', '<', _) => (FlushCounter::Two, Operator::ShiftLeft),
                    ('>', '>', _) => (FlushCounter::Two, Operator::ShiftRight),
                    ('&', '&', _) => (FlushCounter::Two, Operator::LogicalAnd),
                    ('|', '|', _) => (FlushCounter::Two, Operator::LogicalOr),
                    ('<', '=', _) => (FlushCounter::Two, Operator::Le),
                    ('>', '=', _) => (FlushCounter::Two, Operator::Ge),
                    ('=', '=', _) => (FlushCounter::Two, Operator::Equal),
                    ('!', '=', _) => (FlushCounter::Two, Operator::Different),
                    ('+', '=', _) => (FlushCounter::Two, Operator::AddAssign),
                    ('-', '=', _) => (FlushCounter::Two, Operator::SubAssign),
                    ('*', '=', _) => (FlushCounter::Two, Operator::MulAssign),
                    ('/', '=', _) => (FlushCounter::Two, Operator::DivAssign),
                    ('%', '=', _) => (FlushCounter::Two, Operator::ModAssign),
                    ('&', '=', _) => (FlushCounter::Two, Operator::AndAssign),
                    ('|', '=', _) => (FlushCounter::Two, Operator::OrAssign),
                    ('^', '=', _) => (FlushCounter::Two, Operator::XorAssign),
                    ('+', _, _) => (FlushCounter::One, Operator::Plus),
                    ('-', _, _) => (FlushCounter::One, Operator::Minus),
                    ('(', _, _) => (FlushCounter::One, Operator::ParenthesisOpen),
                    (')', _, _) => (FlushCounter::One, Operator::ParenthesisClose),
                    ('[', _, _) => (FlushCounter::One, Operator::BracketOpen),
                    (']', _, _) => (FlushCounter::One, Operator::BracketClose),
                    ('.', _, _) => (FlushCounter::One, Operator::Dot),
                    ('{', _, _) => (FlushCounter::One, Operator::BraceOpen),
                    ('}', _, _) => (FlushCounter::One, Operator::BraceClose),
                    ('~', _, _) => (FlushCounter::One, Operator::BitwiseNot),
                    ('!', _, _) => (FlushCounter::One, Operator::LogicalNot),
                    ('*', _, _) => (FlushCounter::One, Operator::Star),
                    ('&', _, _) => (FlushCounter::One, Operator::Ampersand),
                    ('%', _, _) => (FlushCounter::One, Operator::Modulo),
                    ('/', _, _) => (FlushCounter::One, Operator::Divide),
                    ('>', _, _) => (FlushCounter::One, Operator::Gt),
                    ('<', _, _) => (FlushCounter::One, Operator::Lt),
                    ('=', _, _) => (FlushCounter::One, Operator::Assign),
                    ('|', _, _) => (FlushCounter::One, Operator::BitwiseOr),
                    ('^', _, _) => (FlushCounter::One, Operator::BitwiseXor),
                    (',', _, _) => (FlushCounter::One, Operator::Comma),
                    ('?', _, _) => (FlushCounter::One, Operator::Interrogation),
                    (':', _, _) => (FlushCounter::One, Operator::Colon),
                    (';', _, _) => (FlushCounter::One, Operator::SemiColon),
                    _ => unreachable!(),
                };
                lexed_operator = Some(operator);
                match size {
                    FlushCounter::One => Self::Two(*second, *third),
                    FlushCounter::Two => Self::One(*third),
                    FlushCounter::Three => Self::None,
                }
            }
        };
        lexed_operator
    }
}

#[derive(Default)]
enum TokenBuilderContent {
    /// Identifier, used when parsing function definitions
    Ident(String),
    /// String literal
    String(String),
    /// Char literal. When the first ' is read, this is None.
    Char(Option<char>),
    /// Number literal
    Number(String),
    /// Operator, see https://en.cppreference.com/w/c/language/operator_precedence
    Operator(OperatorBuilder),
    #[default]
    None,
}

impl TokenBuilderContent {
    fn take_token<'filepath>(&mut self) -> Option<Token> {
        match &self {
            Self::Ident(string) => todo!(),
            Self::String(string) => todo!(),
            Self::Char(Some(char)) => Some(Token::Literal(Literal::Char(*char))),
            Self::Number(string) => todo!(),
            Self::Operator(op) => todo!(),
            _ => None,
        }
    }
}

#[derive(Default)]
struct TokenBuilder<'filepath> {
    span: TokenSpan<'filepath>,
    content: TokenBuilderContent,
}

impl<'a, 'filepath: 'a> TokenBuilder<'filepath> {
    fn push_token(&mut self, tokens: &mut Vec<LToken<'a>>) {
        let filepath = self.span.filepath;
        if let Some(token) = self.content.take_token() {
            tokens.push(LToken { span: std::mem::take(&mut self.span), token });
            self.span.filepath = filepath;
        }
    }

    fn lex_char(
        &mut self,
        tokens: &mut Vec<LToken<'filepath>>,
        ch: char,
        location: Location,
    ) -> Result<(), String> {
        match (ch, &mut self.content) {
            // Parse char
            ('\'', TokenBuilderContent::Char(None)) => return Err("missing element in char".into()),
            (_, TokenBuilderContent::Char(ch_builder @ None)) => {
                self.span.start = location;
                *ch_builder = Some(ch)
            }
            // Parse string
            ('\'', TokenBuilderContent::Char(Some(_)))
            | ('"', TokenBuilderContent::String(_)) => self.push_token(tokens),
            (_, TokenBuilderContent::Char(Some(_))) =>
                return Err("more than one element in char".into()),
            (_, TokenBuilderContent::String(string)) => string.push(ch),

            // Parse number
            ('0'..='9' | 'a'..='z' | 'A'..='Z' | '_', TokenBuilderContent::Ident(string))
            | (
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '.' | '+' | '-',
                TokenBuilderContent::Number(string),
            ) => string.push(ch),

            // Parse operator
            (
                '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/' | '>' | '<'
                | '=' | '|' | '^' | ',' | '?' | ':' | ';' | '.' | '+' | '-',
                TokenBuilderContent::Operator(op),
            ) =>
                if let Some(operator) = op.push(ch) {
                    self.push_token(tokens);
                    // tokens.push(LToken {
                    //     span: self.span,
                    //     token: Token::Operator(operator),
                    // });
                },
            (
                _,
                TokenBuilderContent::Number(_)
                | TokenBuilderContent::Ident(..)
                | TokenBuilderContent::Operator(_),
            ) => return Err("invalid character".into()),
            (_, TokenBuilderContent::None) => todo!(),
        }
        Ok(())
    }
}

/// lexicalize the provided lines. It is the responsability of the user to
/// ensure that lines belong to filepath.
pub fn lex<'b, 'filepath: 'b>(
    filepath: &'filepath Path,
    lines: Lines<'_>,
) -> Result<Tokens<'b>, String> {
    let mut tokens = vec![];
    let mut builder = TokenBuilder::default();
    let mut location = Location::from(filepath);
    for line in lines {
        for ch in line.chars() {
            builder.lex_char(&mut tokens, ch, location.clone())?;
            location.incr_col();
        }
        location.incr_line();
    }
    Ok(tokens)
}
