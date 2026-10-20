use crate::lexer::center::lex::Lexer;

impl Lexer{

    pub fn eat(&mut self) -> &str{
        let curr = self.tokenizer.current as usize;
        self.tokenizer.current += 1;

        &self.tokenizer.input[curr..=curr]
    }
    pub fn peek(&self) -> &str{
        &self.tokenizer.input[self.tokenizer.current as usize..=self.tokenizer.current as usize]

    }
    pub fn peek_next(&mut self) -> &str{
        &self.tokenizer.input[(self.tokenizer.current + 1) as usize ..= (self.tokenizer.current + 1) as usize]
    }

    pub fn is_finish(&self) -> bool{
        &self.tokenizer.input[self.tokenizer.current as usize..=self.tokenizer.current as usize] == "\0"
    }
}

