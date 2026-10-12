use crate::lex::token::Token;
use crate::lex::iterval::Interval;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer{
    pub input    : String,
    pub intervals: Vec<Interval>,
    pub tokens   : Vec<Token>,
    pub lines    : Vec<u32>,
    start        : u32,
    current      : u32,
    line         : u32
}

impl Lexer{
    pub fn init(input : String) -> Self{
		let alloca_size = 1024;

        Lexer{
            input,
            intervals: Vec::with_capacity(alloca_size),
            tokens   : Vec::with_capacity(alloca_size),
            lines    : Vec::with_capacity(alloca_size),
            start    : 0,
            current  : 0,
            line     : 1,
        }
    }

    pub fn tokenize(&mut self){
        while !self.finish(){
            self.start = self.current;
			let c = self.eat();
			match c{
				"+" => {
					self.tokens.push(Token::Plus);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},

				"-" => {
					if self.peek() == ">"{
						self.tokens.push(Token::Arrow);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					}else{
						self.tokens.push(Token::Minus);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					}
				},

				"*" => {
					self.tokens.push(Token::Mul);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"/" => {
					self.tokens.push(Token::Div);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"%" => {
					self.tokens.push(Token::Mod);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"=" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::EqEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					} else{
						self.tokens.push(Token::Equel);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					}

				},
				">" => {
					if self.peek() == "="{
						self.tokens.push(Token::GrtEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					} else{
						self.tokens.push(Token::Grt);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					}
				},
				"<" => {
					if self.peek() == "="{
						self.tokens.push(Token::LesEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					} else{
						self.tokens.push(Token::Les);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					}
				},
				"!" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::NotEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
					} else{
						panic!("[Line {:?}] '!' must combine with '=' ", self.line)
					}
				},
				"(" => {
					self.tokens.push(Token::Lparen);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				")" => {
					self.tokens.push(Token::Rparen);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"{" => {
					self.tokens.push(Token::Lbrace);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"}" => {
					self.tokens.push(Token::Rbrace);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				":" => {
					self.tokens.push(Token::Colon);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				";" => {
					self.tokens.push(Token::Scolon);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"," => {
					self.tokens.push(Token::Comma);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"." => {
					self.tokens.push(Token::Dot);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"[" => {
					self.tokens.push(Token::Lbracket);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"]" => {
					self.tokens.push(Token::Rbracket);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},
				"|" => {
					self.tokens.push(Token::Vartical);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
				},

				_ if is_number(c) => self.number(),

				_ if is_alphanumeric(c) => self.ident(),

				" " | "\r" | "\t" => {},

				"\n" => self.line += 1,

				e => panic!("{}",format!("Find Unknown Token : {e}"))
			}
        }
        self.tokens.push(Token::EOF);
    } 


    
    fn number(&mut self){
	    while is_number(self.peek()){self.current += 1}
	    
	    if matches!(self.peek(), ".") && is_number(self.peek_next()){
	        self.current += 1;
	        while is_number(self.peek()){self.current += 1}
	        
            
	        self.tokens.push(Token::Float);
            self.intervals.push(Interval::new(self.start, self.current));
            self.lines.push(self.line);
	        
	        
	    }else{
	        self.tokens.push(Token::Int);
            self.intervals.push(Interval::new(self.start, self.current));
            self.lines.push(self.line);
	    }
    }
    
    
    fn ident(&mut self){
	    while is_alphanumeric(self.peek()) {self.current += 1}
	    let slice = &self.input[self.start as usize..self.current as usize];
	    match slice{
	        "let"      => {
		        self.tokens.push(Token::Let);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "for"      => {
		        self.tokens.push(Token::For);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "in"       => {
		        self.tokens.push(Token::In);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "while"    => {
		        self.tokens.push(Token::While);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "continue" =>{
		        self.tokens.push(Token::Ctn);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        } 
	        "break"    => {
		        self.tokens.push(Token::Brk);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "if"       => {
		        self.tokens.push(Token::If);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "elif"     => {
		        self.tokens.push(Token::Elif);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "else"     => {
		        self.tokens.push(Token::Else);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "not"      => {
		        self.tokens.push(Token::CmpNot);
		        self.intervals.push(Interval::new(self.start, self.current));
            self.lines.push(self.line);
	        }
	        "and"      => {
		        self.tokens.push(Token::CmpAnd);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "or"       => {
		        self.tokens.push(Token::CmpOr);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "fn"       => {
		        self.tokens.push(Token::Func);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "return"   => {
		        self.tokens.push(Token::Ret);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "true"     => {
		        self.tokens.push(Token::True);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
	        "false"     => {
		        self.tokens.push(Token::False);
		        self.intervals.push(Interval::new(self.start, self.current));
                self.lines.push(self.line);
	        }
			"i32" | "i64" | "f32" | "f64" | "bool" => {
				self.tokens.push(Token::Type);
				self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line)
			},
	        _          => {
		        self.tokens.push(Token::Ident);
		        self.intervals.push(Interval::new(self.start, self.current));
		        self.lines.push(self.line);
	        }
	        
	    }
    }

	#[inline(always)]
    fn eat(&mut self) -> &str{
	    let curr = (self.current) as usize;
        self.current += 1;
	    
        &self.input[curr..=curr]
    }
	#[inline(always)]
    fn peek(&mut self) -> &str{
	    if most_freq(!self.finish()) {
            return &self.input[(self.current) as usize..=(self.current) as usize];
        }
		"\0" // less_freq

    }
	#[inline(always)]
    fn peek_next(&mut self) -> &str{
	    if most_freq(!self.finish()) {
            return &self.input[(self.current + 1) as usize ..= (self.current + 1) as usize];
        }
		"\0"
        

    }
    
	#[inline(always)]
    fn finish(&mut self) -> bool{
		self.current >= self.input.len().try_into().unwrap()
    }
    
    pub fn print_output(&mut self){
	    println!("Tokens   : {:?}", &self.tokens);
	    println!("Lines    : {:?}", &self.lines);
	    println!("Intervals : {:?}", &self.intervals);
	    for (i, interval) in self.intervals.iter().enumerate(){
	        let start = interval.start as usize;
	        let end   = interval.end   as usize;
	        let slice = &self.input[start..end];
	        println!("number : {i}, {slice} ");
	    }
    }
    
} 


fn is_number(c : &str) -> bool{
    c >= "0" && c <= "9"
}

fn is_alphanumeric (c : &str) -> bool{
    (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || (c >= "0" && c <= "9")
}

#[inline(always)]
fn most_freq(cond : bool) -> bool{
	cond
}



#[test]
fn lexer_testing(){
    let input ="let x = 1".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();
}
