use std::fs;
use std::env;
use std::panic;
enum TokenKind{

    



}

struct Token{


    token_type:TokenKind,

    literal: String,



}



#[derive(Debug)]
struct Lexer {


    source : Vec<char>,
    counter : usize,

}

impl Lexer {

    pub fn new(contents:String)->Self{

        Self{
            source:contents.chars().collect(),

            counter:0,
    
        }


    }

    pub fn lex(&mut self){


        println!("{}", self.contents);


    }

}
fn main() {

    let maybe_file = env::args().nth(1);

    let file= if let Some(f) =  maybe_file{


        f 


    }else {
        
        panic!("Expected a file.")        


    };


    let maybe_contents  = fs::read_to_string(file);   


    let contents = if maybe_contents.is_ok(){


        maybe_contents.unwrap()
        
    }else{


        panic!("couldn't open file for reading ");

    };
    
    let mut lexer = Lexer::new(contents);   
    

    lexer.lex();

}
