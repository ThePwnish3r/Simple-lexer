use std::env;
use std::fs;
use std::panic;


#[derive(Debug)]
enum TokenKind {
    Identifier,

    Assign,

    Let,

    String,
}
#[derive(Debug)]
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
        let mut tokens: Vec<Token> = Vec::new();

        while self.source.len() > self.counter {

            let mut c = self.current_char();


            match  c {
                
                '=' => {

                    tokens.push(Token::new(TokenKind::Assign, "=".to_owned()));

                    self.counter += 1


                },   
                
                
                '\''| '"'=>{

                    self.counter += 1;



                    let mut buffer : String = String::new();


                    while self.current_char() != c{


                        buffer.push(self.current_char());

                        self.counter +=1 ;



                    }

                    self.counter +=1 ;

                    tokens.push(Token::new(TokenKind::String, buffer));

                },

                _ if c.is_alphabetic() => {

                    let mut buffer:String = String::new();


                    buffer.push(c);

                    self.counter +=1 ;


                    while self.current_char().is_alphabetic(){

                        buffer.push(self.current_char());

                        self.counter += 1 ;

                    }

                    let kind : TokenKind = match buffer.as_str(){

                        "let" => {
                            TokenKind::Let
                        },
                        _ => TokenKind::Identifier,

                    };

                    tokens.push(Token::new(kind,buffer));
                }

                _=> {

                    self.counter += 1


                }

            }


        }

        println!("{:?}",tokens);
    }


    pub fn current_char(&self)-> char{

        *self.source.get(self.counter).unwrap()



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
