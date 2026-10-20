use crate::lexer::tokenizer::source::token::Token;
use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::source::data::{SpanLength, SpanStart};

impl Lexer{

    pub fn tokenize(&mut self){
        while !self.is_finish(){
            self.tokenizer.start = self.tokenizer.current;
            let c = self.eat();
            match c{
                "+" => {
                    if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::PlusEq);
                    }else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Plus);
                    }
                },

                "-" => {
                    if matches!(self.peek(), ">"){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Arrow);
                    }else if matches!(self.peek(), "="){
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::MinusEq);
                    }else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Minus);
                    }
                },

                "*" => {
                    if matches!(self.peek(), ">"){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::MulEq);
                    }else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Mul);
                    }
                },
                "/" => {
                    if matches!(self.peek(), "/"){
                        while !matches!(self.peek(), "\n")&& !self.is_finish(){
                            self.eat();
                        }
                    }else if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::DivEq);
                    }else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Div);
                    }
                },
                "%" => {
                    if matches!(self.peek(), ">"){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::ModEq);
                    }else {
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Mod);
                    }
                },
                "=" => {
                    if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::EqEq);
                    } else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Eq);
                    }

                },
                ">" => {
                    if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::GrtEq);
                    } else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Grt);
                    }
                },
                "<" => {
                    if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::LesEq);
                    } else{
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::Les);
                    }
                },
                "!" => {
                    if matches!(self.peek(), "="){
                        self.eat();
                        self.collect_diagnostic_data();
                        self.collect_tokenizer_data(Token::NotEq);
                    } else{
                        self.collection_illegal_issue();
                        self.eat();
                    }
                },
                "(" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Lparen);
                },
                ")" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Rparen);
                },
                "{" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Lbrace);
                },
                "}" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Rbrace);
                },
                ":" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Colon);
                },
                ";" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Scolon);
                },
                "," => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Comma);
                },
                "[" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Lbracket);
                },
                "]" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Rbracket);
                },
                "|" => {
                    self.collect_diagnostic_data();
                    self.collect_tokenizer_data(Token::Vertical);
                },

                _ if is_number(c) => self.number(),

                _ if is_alphanumeric(c) => self.ident(),

                "\"" => self.string(),

                " " | "\r" | "\t" => {},

                "\n" => {
                    self.error.spans.push(
                        (self.error.span_start as SpanStart
                         , self.tokenizer.current as SpanLength
                        )
                    );
                    self.error.span_start = self.tokenizer.current;
                    self.error.span_id += 1;
                    self.error.line += 1;
                },

                _ => {
                    self.collection_illegal_issue();
                    self.eat();
                }
            }
        }
        self.tokenizer.tokens.push(Token::EOF);
        self.tokenizer.tokens.push(Token::EOF);
        self.tokenizer.tokens.push(Token::EOF);

        if self.error.has_error{
            self.output_error();
        }
    }



    fn number(&mut self){
        while is_number(self.peek()){self.eat();}

        if matches!(self.peek(), ".") && is_number(self.peek_next()){
            self.eat();
            while is_number(self.peek()){self.eat();}


            self.collect_diagnostic_data();
            self.collect_tokenizer_data(Token::Float);
        }else{
            self.collect_diagnostic_data();
            self.collect_tokenizer_data(Token::Int);
        }
    }


    fn ident(&mut self){
        while is_alphanumeric(self.peek()) {self.eat();}
        let slice = &self.tokenizer.input[self.tokenizer.start as usize..self.tokenizer.current as usize];
        match slice{
            "val"      => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Val);
            }
            "var"      => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Var);
            },
            "for"      => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::For);
            }
            "in"       => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::In);
            }
            "while"    => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::While);
            }
            "continue" =>{
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Ctn);
            }
            "break"    => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Brk);
            }
            "if"       => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::If);
            }
            "elif"     => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Elif);
            }
            "else"     => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Else);
            }
            "not"      => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::CmpNot);
            }
            "and"      => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::CmpAnd);
            }
            "or"       => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::CmpOr);
            }
            "fn"       => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Func);
            }
            "return"   => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Ret);
            }
            "true"     => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::True);
            }
            "false"     => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::False);
            },
            "print"    => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Print);
            },
            "void"|"i8" | "i16" | "i32" | "i64" |
            "f32" | "f64" | "bool" | "String" => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Type);
            },
            _          => {
                self.collect_diagnostic_data();
                self.collect_tokenizer_data(Token::Ident);
            }

        }
    }


    fn string(&mut self){
        while self.peek() != "\"" && !self.is_finish(){
            if self.peek() != "\n"{
                self.eat();
            }else{
                self.error.line += 1;
                self.error.span_id += 1;
                self.eat();
            }
        }

        if self.is_finish(){
            panic!("[Line {}] Unterminated string.", self.error.line);
        }

        //the closing
        self.eat();

        self.collect_diagnostic_data();
        self.collect_tokenizer_data(Token::String);

    }
    
    
    
    
    
    
    
}

pub fn is_number(c : &str) -> bool{
    c >= "0" && c <= "9"
}

pub fn is_alphanumeric (c : &str) -> bool{
    (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || (c >= "0" && c <= "9") || c == "_"
}

