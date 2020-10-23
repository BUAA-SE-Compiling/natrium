use logos::{Lexer, Logos};
use smol_str::SmolStr;

fn parse_string_literal(i: &mut Lexer<Token>) -> Option<String> {
    unescape::unescape(&i.slice()[1..i.slice().len() - 1])
    // Some(i.slice().into())
}

#[derive(Debug, Clone, Logos)]
pub enum Token {
    #[token("fn")]
    FnKw,
    #[token("let")]
    LetKw,
    #[token("const")]
    ConstKw,
    #[token("as")]
    AsKw,
    #[token("while")]
    WhileKw,
    #[token("if")]
    IfKw,
    #[token("else")]
    ElseKw,

    #[regex(r"\d+", |lex| lex.slice().parse(), priority = 2)]
    UIntLiteral(u64),
    #[regex(r"\d+(.\d+)?([eE]\d+)?", |lex| lex.slice().parse())]
    FloatLiteral(f64),
    #[regex(r#""([^\\"]|\\([rnt\\/"']))*""#, parse_string_literal, priority = 2)]
    StringLiteral(String),
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| SmolStr::new(lex.slice()))]
    Ident(SmolStr),

    #[token(r"+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"*")]
    Mul,
    #[token(r"/")]
    Div,
    #[token(r"=")]
    Assign,
    #[token(r"==")]
    Eq,
    #[token(r"!=")]
    NEq,
    #[token(r"<")]
    Lt,
    #[token(r">")]
    Gt,
    #[token(r"<=")]
    Le,
    #[token(r">=")]
    Ge,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(r"->")]
    Arrow,
    #[token(r",")]
    Comma,
    #[token(r":")]
    Colon,
    #[token(r";")]
    Semicolon,

    // Empty stuff
    #[regex(r"\s+", logos::skip, priority = 1)]
    Whitespace,
    #[regex(r"//.*\n", logos::skip)]
    Comment,

    // Error token
    #[error]
    Error,
}

impl Token {
    pub fn get_ident(&self) -> Option<&str> {
        match self {
            Token::Ident(i) => Some(&i),
            _ => None,
        }
    }

    pub fn get_ident_owned(self) -> Option<SmolStr> {
        match self {
            Token::Ident(i) => Some(i),
            _ => None,
        }
    }

    pub fn get_uint(&self) -> Option<u64> {
        match self {
            Token::UIntLiteral(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        match self {
            Token::FloatLiteral(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&str> {
        match self {
            Token::StringLiteral(i) => Some(&i),
            _ => None,
        }
    }

    pub fn get_string_owned(self) -> Option<String> {
        match self {
            Token::StringLiteral(i) => Some(i),
            _ => None,
        }
    }
}
