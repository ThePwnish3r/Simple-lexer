use std::env;
use std::fs;
use std::panic;
enum TokenKind {
    Identifier,

    Assign,

    Let,

    String,
}

struct Token {
    kind: TokenKind,

    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
struct Lexer {
    source: Vec<char>,
    counter: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),

            counter: 0,
        }
    }

    pub fn lex(&mut self) {
        let tokens: Vec<Token> = Vec::new();

        while self.source.len() > self.counter {


            



        }
    }
}
fn main() {
    let maybe_file = env::args().nth(1);

    let file = if let Some(f) = maybe_file {
        f
    } else {
        panic!("Expected a file.")
    };

    let maybe_contents = fs::read_to_string(file);

    let contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("couldn't open file for reading ");
    };

    let mut lexer = Lexer::new(contents);

    lexer.lex();
}
