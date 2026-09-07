use crate::lex::token::Token;
use crate::lex::iterval::Iterval;


#[derive(Debug, Clone, PartialEq)]
pub struct Lexer{
    input    : String,
    itervals : Vec<Iterval>,
    tokens   : Vec<Token>,
    lines    : Vec<u32>,
    start    : usize,
    current  : usize,
    line     : u32
}

impl Lexer{
    pub fn init(input : String) -> Self{
        Lexer{
            input,
            itervals : Vec::new(),
            tokens   : Vec::new(),
            lines    : Vec::new(),
            start    : 0,
            current  : 0,
            line     : 1,
        }
    }

    pub fn tokenize(&mut self){
        while self.current < self.input.len().try_into().unwrap(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::EOF);


    } 

    fn scan_token(&mut self){
        let cur = self.current as u32;
        let c = self.eat();

        match c{
            "+" => {
                self.tokens.push(Token::Plus);
                self.itervals.push(Iterval::init(cur, cur));
                self.lines.push(self.line);
            },

            "-" => {
                self.tokens.push(Token::Minus);
                self.itervals.push(Iterval::init(cur, cur));
                self.lines.push(self.line);
            },

            "*" => {
                self.tokens.push(Token::Mul);
                self.lines.push(self.line);
                self.itervals.push(Iterval::init(cur, cur));
            }, 
            "/" => {
                self.tokens.push(Token::Div);
                self.itervals.push(Iterval::init(cur, cur));
                self.lines.push(self.line);
            },
            "%" => {
                self.tokens.push(Token::Mod);
                self.itervals.push(Iterval::init(cur, cur));
                self.lines.push(self.line);
            },
            "=" => {
                self.tokens.push(Token::Equel);
                self.itervals.push(Iterval::init(cur, cur));
                self.lines.push(self.line);

            },
            ">" => {
                if matches!(self.peek_next(), "="){
                    self.tokens.push(Token::GrtEq);
                    self.itervals.push(Iterval::init(cur, cur));
                    self.lines.push(self.line);
                    self.current += 1;
                } else{
                    self.tokens.push(Token::Grt);
                    self.itervals.push(Iterval::init(cur, cur));
                    self.lines.push(self.line);
                }
            },
            "<" => {
                if matches!(self.peek_next(), "="){
                    self.tokens.push(Token::LesEq);
                    self.itervals.push(Iterval::init(cur, cur));
                    self.lines.push(self.line);
                    self.current += 1;
                }

            }





            " " | "/r" | "/t" => unreachable!(),
            "/n" => self.line += 1,
            _ => panic!("fuck")

            }
        
        self.tokens.push(Token::EOF);
    }


    fn eat(&mut self) -> &str{
        let current = self.current;
        self.current += 1;
        &self.input[current..current]
    }

    fn peek_next(&mut self) -> &str{
        &self.input[self.current..self.current]
    }

}
