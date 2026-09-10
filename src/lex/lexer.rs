use crate::lex::token::Token;
use crate::lex::iterval::Iterval;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer{
    pub input    : String,
    pub itervals : Vec<Iterval>, // start, end
    pub tokens   : Vec<Token>,
    pub lines    : Vec<u32>,
    start        : u32,
    current      : u32,
    line         : u32
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
        while !self.finish(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::EOF);
    } 

    fn scan_token(&mut self){
        let c = self.eat();
	match c{
	    "+" => {
                    self.tokens.push(Token::Plus);
		    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
            },

            "-" => {
		if matches!(self.peek(), ">"){
		    self.tokens.push(Token::Arrow);
		    self.current += 1;
		    self.itervals.push(Iterval::init(self.start, self.current));
		    self.lines.push(self.line);
		} else{
                    self.tokens.push(Token::Minus);
		    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
                }
            },

            "*" => {
                    self.tokens.push(Token::Mul);
		    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
            }, 
            "/" => {
                    self.tokens.push(Token::Div);
		    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
            },
	    "%" => {
                    self.tokens.push(Token::Mod);
		    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
            },
            "=" => {
                 if matches!(self.peek(), "="){
                     self.tokens.push(Token::EqEq);
                     self.current += 1;
		     self.itervals.push(Iterval::init(self.start, self.current));
                     self.lines.push(self.line);
                } else{
                     self.tokens.push(Token::Equel);
                     self.itervals.push(Iterval::init(self.start, self.current));
                     self.lines.push(self.line);
                }

            },
            ">" => {
                if matches!(self.peek(), "="){
                    self.tokens.push(Token::GrtEq);
                    self.current += 1;
		    self.itervals.push(Iterval::init(self.start, self.current));
		    self.lines.push(self.line);
                } else{
                    self.tokens.push(Token::Grt);
                    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
                }
            },
            "<" => {
		 if matches!(self.peek(), "="){
                     self.tokens.push(Token::LesEq);
		     self.current += 1;
                     self.itervals.push(Iterval::init(self.start, self.current));
                     self.lines.push(self.line);
                 } else{
		     self.tokens.push(Token::Les);
		     self.itervals.push(Iterval::init(self.start, self.current));
                     self.lines.push(self.line);
                 }
            },
	    "!" => {
		if matches!(self.peek(), "="){
		    self.tokens.push(Token::NotEq);
                    self.current += 1;
                    self.itervals.push(Iterval::init(self.start, self.current));
                    self.lines.push(self.line);
		} else{
		    panic!("[Line {:?}] '!' must combine with '=' ", self.line)
		}
	    },
	    "(" => {
		self.tokens.push(Token::Lparen);
		self.itervals.push(Iterval::init(self.start, self.current));
		self.lines.push(self.line);
	    },
	    ")" => {
		self.tokens.push(Token::Rparen);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    "{" => {
		self.tokens.push(Token::Lbrace);
		self.itervals.push(Iterval::init(self.start, self.current));
		self.lines.push(self.line);
	    },
	    "}" => {
		self.tokens.push(Token::Rbrace);
		self.itervals.push(Iterval::init(self.start, self.current));
		self.lines.push(self.line);
	    },
	    ":" => {
		self.tokens.push(Token::Colon);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    ";" => {
		self.tokens.push(Token::Scolon);
                self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    "," => {
		self.tokens.push(Token::Comma);
		self.itervals.push(Iterval::init(self.start, self.current));
		self.lines.push(self.line);
	    },
	    "." => {
		self.tokens.push(Token::Dot);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    "[" => {
		self.tokens.push(Token::Lbracket);
                self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    "]" => {
		self.tokens.push(Token::Rbracket);
                self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    "|" => {
		self.tokens.push(Token::Vartical);
                self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    },
	    
	    _ if is_number(c) => self.number(),
		
	    _ if is_alphanumeric(c) => self.ident(),
	    
            " " | "\r" | "\t" => {},
	    
            "\n" => self.line += 1,
            
            e => panic!("{}",format!("Find Unknown Token : {e}"))
            }
        
    }

    fn number(&mut self){
	while is_number(self.peek()){self.current += 1}
	
	if matches!(self.peek(), ".") && is_number(self.peek_next()){
	    self.current += 1;
	    while is_number(self.peek()){self.current += 1}
	    

	    self.tokens.push(Token::Int);
            self.itervals.push(Iterval::init(self.start, self.current));
            self.lines.push(self.line);
	    
	   
	} else{

	    self.tokens.push(Token::Int);
            self.itervals.push(Iterval::init(self.start, self.current));
            self.lines.push(self.line);
	}
    }

    
    fn ident(&mut self){
	while is_alphanumeric(self.peek()) {self.current += 1}
	let slice = &self.input[as_usize(self.start)..as_usize(self.current)];
	match slice{
	    "let"      => {
		self.tokens.push(Token::Let);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "for"      => {
		self.tokens.push(Token::For);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "in"       => {
		self.tokens.push(Token::In);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "while"    => {
		self.tokens.push(Token::While);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "continue" =>{
		self.tokens.push(Token::Ctn);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    } 
	    "break"    => {
		self.tokens.push(Token::Brk);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "if"       => {
		self.tokens.push(Token::If);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "elif"     => {
		self.tokens.push(Token::Elif);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "else"     => {
		self.tokens.push(Token::Else);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "not"      => {
		self.tokens.push(Token::CmpNot);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "and"      => {
		self.tokens.push(Token::CmpAnd);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "or"       => {
		self.tokens.push(Token::CmpOr);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "fn"       => {
		self.tokens.push(Token::Func);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "return"   => {
		self.tokens.push(Token::Ret);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "true"     => {
		self.tokens.push(Token::True);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    "false"     => {
		self.tokens.push(Token::False);
		self.itervals.push(Iterval::init(self.start, self.current));
                self.lines.push(self.line);
	    }
	    _          => {
		self.tokens.push(Token::Ident);
		self.itervals.push(Iterval::init(self.start, self.current));
		self.lines.push(self.line);
	    }
	    
	}
    }


    fn eat(&mut self) -> &str{
	let curr = as_usize(self.current);
        self.current += 1;
	
        &self.input[curr..=curr]
    }

    fn peek(&mut self) -> &str{
	if self.finish() {
            return "\0";
        }
        &self.input[as_usize(self.current)..=as_usize(self.current)]
    }

    fn peek_next(&mut self) -> &str{
	if self.finish() {
            return "\0";
        }

	&self.input[as_usize(self.current + 1) ..= as_usize(self.current + 1)]
    }

    fn finish(&mut self) -> bool{
	self.current >= self.input.len().try_into().unwrap()
    }

    pub fn print_output(&mut self){
	println!("Tokens   : {:?}", &self.tokens);
	println!("Lines    : {:?}", &self.lines);
	println!("Itervals : {:?}", &self.itervals);
	for (i, iterval) in self.itervals.iter().enumerate(){
	    let start = iterval.start as usize;
	    let end   = iterval.end   as usize;
	    let slice = &self.input[start..end];
	    println!("number : {i}, {slice} ");
	}
    }

} 


fn as_usize(idx : u32) -> usize{
    idx as usize
}

fn is_number(c : &str) -> bool{
    c >= "0" && c <= "9"
}

fn is_alphanumeric (c : &str) -> bool{
    (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || (c >= "0" && c <= "9")
}

#[test]
fn lex_testing(){
    let input ="'".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();
}
