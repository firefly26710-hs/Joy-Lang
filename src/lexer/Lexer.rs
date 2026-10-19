use crate::lexer::diagnostic::error::Error;
use crate::lexer::interval::Interval;
use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]

pub struct Lexer {
	pub input    : String,
    pub intervals: Vec<Interval>,
    pub tokens   : Vec<Token>,
    pub lines    : Vec<u32>,
    pub start    : u32,
    pub current  : u32,
    pub line     : u32,


	pub error     : Error
}

impl Lexer {
    pub fn init(input : String, file_name : String) -> Self{
		let alloc_size = input.len();

        Lexer {
            input,
            intervals: Vec::with_capacity(alloc_size),
            tokens   : Vec::with_capacity(alloc_size),
            lines    : Vec::with_capacity(alloc_size),
            start    : 0,
            current  : 0,
            line     : 1,

			error    : Error::init(file_name, alloc_size)
        }
    }

    pub fn tokenize(&mut self){
        while !self.is_finish(){
            self.start = self.current;
			let c = self.eat();
			match c{
				"+" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::PlusEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else{
						self.tokens.push(Token::Plus);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},

				"-" => {
					if matches!(self.peek(), ">"){
						self.tokens.push(Token::Arrow);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else if matches!(self.peek(), "="){
						self.tokens.push(Token::MinusEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else{
						self.tokens.push(Token::Minus);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},

				"*" => {
					if matches!(self.peek(), ">"){
						self.tokens.push(Token::MulEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else{
						self.tokens.push(Token::Mul);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},
				"/" => {
					if matches!(self.peek(), "/"){
						while !matches!(self.peek(), "\n")&& !self.is_finish(){
							self.eat();
						}
					}else if matches!(self.peek(), "="){
						self.tokens.push(Token::DivEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else{
						self.tokens.push(Token::Div);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},
				"%" => {
					if matches!(self.peek(), ">"){
						self.tokens.push(Token::ModEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}else {
						self.tokens.push(Token::Mod);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},
				"=" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::EqEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					} else{
						self.tokens.push(Token::Equal);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}

				},
				">" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::GrtEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					} else{
						self.tokens.push(Token::Grt);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},
				"<" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::LesEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					} else{
						self.tokens.push(Token::Les);
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					}
				},
				"!" => {
					if matches!(self.peek(), "="){
						self.tokens.push(Token::NotEq);
						self.current += 1;
						self.intervals.push(Interval::new(self.start, self.current));
						self.lines.push(self.line);
						self.error.span_ids.push(self.error.span_id);
					} else{
						panic!("[Line {:?}] '!' must combine with '=' ", self.line)
					}
				},
				"(" => {
					self.tokens.push(Token::Lparen);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				")" => {
					self.tokens.push(Token::Rparen);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"{" => {
					self.tokens.push(Token::Lbrace);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"}" => {
					self.tokens.push(Token::Rbrace);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				":" => {
					self.tokens.push(Token::Colon);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				";" => {
					self.tokens.push(Token::Scolon);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"," => {
					self.tokens.push(Token::Comma);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"." => {
					self.tokens.push(Token::Dot);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"[" => {
					self.tokens.push(Token::Lbracket);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"]" => {
					self.tokens.push(Token::Rbracket);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},
				"|" => {
					self.tokens.push(Token::Vertical);
					self.intervals.push(Interval::new(self.start, self.current));
					self.lines.push(self.line);
					self.error.span_ids.push(self.error.span_id);
				},

				_ if is_number(c) => self.number(),

				_ if is_alphanumeric(c) => self.ident(),

				"\"" => self.string(),

				" " | "\r" | "\t" => {},

				"\n" => {
					self.error.spans.push((self.error.spans_start, self.current));
					self.error.spans_start = self.current;

					self.error.span_id += 1;
					self.line += 1;
				},

				e => panic!("{}",format!("Find Unknown Token : {e}"))
			}
        }
        self.tokens.push(Token::EOF);
		self.tokens.push(Token::EOF);
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
			self.error.span_ids.push(self.error.span_id);
	    }else{
	        self.tokens.push(Token::Int);
            self.intervals.push(Interval::new(self.start, self.current));
			self.lines.push(self.line);
			self.error.span_ids.push(self.error.span_id);
	    }
    }
    
    
    fn ident(&mut self){
	    while is_alphanumeric(self.peek()) {self.current += 1}
	    let slice = &self.input[self.start as usize..self.current as usize];
	    match slice{
			"val"      => {
				self.tokens.push(Token::Val);
				self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
			}
	        "var"      => {
		        self.tokens.push(Token::Var);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        },
	        "for"      => {
		        self.tokens.push(Token::For);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "in"       => {
		        self.tokens.push(Token::In);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "while"    => {
		        self.tokens.push(Token::While);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "continue" =>{
		        self.tokens.push(Token::Ctn);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        } 
	        "break"    => {
		        self.tokens.push(Token::Brk);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "if"       => {
		        self.tokens.push(Token::If);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "elif"     => {
		        self.tokens.push(Token::Elif);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "else"     => {
		        self.tokens.push(Token::Else);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "not"      => {
		        self.tokens.push(Token::CmpNot);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "and"      => {
		        self.tokens.push(Token::CmpAnd);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "or"       => {
		        self.tokens.push(Token::CmpOr);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "fn"       => {
		        self.tokens.push(Token::Func);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "return"   => {
		        self.tokens.push(Token::Ret);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "true"     => {
		        self.tokens.push(Token::True);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        "false"     => {
		        self.tokens.push(Token::False);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        },
			"print"    => {
				self.tokens.push(Token::Print);
				self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
			},
			"void"|"i8" | "i16" | "i32" | "i64" |
			"f32" | "f64" | "bool" | "String" => {
				self.tokens.push(Token::Type);
				self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
			},
	        _          => {
		        self.tokens.push(Token::Ident);
		        self.intervals.push(Interval::new(self.start, self.current));
				self.lines.push(self.line);
				self.error.span_ids.push(self.error.span_id);
	        }
	        
	    }
    }


	fn string(&mut self){
		while self.peek() != "\"" && !self.is_finish(){
			if self.peek() != "\n"{
				self.eat();
			}else{
				self.line += 1;
				self.error.span_id += 1;
				self.eat();
			}
		}

		if self.is_finish(){
			panic!("[Line {}] Unterminated string.", self.line);
		}

		//the closing
		self.eat();

		self.tokens.push(Token::String);
		self.intervals.push(Interval::new(self.start + 1, self.current - 1));
		self.lines.push(self.line);

	}
	

	#[inline(always)]
    fn eat(&mut self) -> &str{
	    let curr = (self.current) as usize;
        self.current += 1;
	    
        &self.input[curr..=curr]
    }
	#[inline(always)]
    fn peek(&self) -> &str{
		&self.input[self.current as usize..=self.current as usize]

    }
	#[inline(always)]
    fn peek_next(&mut self) -> &str{
            &self.input[(self.current + 1) as usize ..= (self.current + 1) as usize]
    }
    
	#[inline(always)]
    fn is_finish(&self) -> bool{
		&self.input[self.current as usize..=self.current as usize] == "\0"
    }
	
    
    pub fn print_output(&mut self){
	    for (i, interval) in self.intervals.iter().enumerate(){
	        let start = interval.start as usize;
	        let end   = interval.end   as usize;
	        let slice = &self.input[start..end];
	        println!("number : {i}, {slice} ");
	    }

		for (i, interval) in self.error.spans.iter().enumerate(){
			let start = interval.0 as usize;
			let end   = interval.1  as usize;
			let slice = &self.input[start..end];
			println!("number : {i}, {slice} ");
		}
    }
    
} 


fn is_number(c : &str) -> bool{
    c >= "0" && c <= "9"
}

fn is_alphanumeric (c : &str) -> bool{
    (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || (c >= "0" && c <= "9") || c == "_"
}





