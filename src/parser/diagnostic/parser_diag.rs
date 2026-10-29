use crate::parser::stmt::logic::expr::is_op;
use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::diagnostic::source::msg_data::MsgData;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;


impl Parser{
    pub fn collect_unexpected_token_issue(
        &mut self,
        correct_token : &Token,
    ) -> IssueType{
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let(line,
            rlt_col_start,
            rlt_col_end,
            alt_col_start,
            alt_col_end,
            span_id)
        = self.get_data(wrong_tok_key);

        let wrong_string = self.lexer.tokenizer.input[alt_col_start..alt_col_end].to_string();
        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue = IssueType::UnExpectedToken{wrong_string, correct_string};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_unclosed_delimiter_token_issue(&mut self) -> IssueType{
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(wrong_tok_key);


        let issue = IssueType::UnclosedDelimiter;

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_mismatch_type_issue(&mut self, expected_type : &Type, actual_type : &Type) -> IssueType{
        self.lexer.error.has_error = true;

        let actual_type_idx = self.current; // 記得要改

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(actual_type_idx);

        let expected_type = from_type_to_string(expected_type).to_string();
        let actual_type = from_type_to_string(actual_type).to_string();

        let issue = IssueType::MismatchesType {expected_type, actual_type};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_variable_not_in_scope_issue(&mut self, key: Key ) -> IssueType{
        self.lexer.error.has_error = true;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(key as usize);

        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_var = self.lexer.tokenizer.input[start..end].to_string();

        let issue = IssueType::VariableNotInScope{unknown_var};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue

    }

    pub fn collect_cannot_found_function_issue(&mut self, key: Key) -> IssueType{
        self.lexer.error.has_error = true;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(key as usize);

        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_fn = self.lexer.tokenizer.input[start..end].to_string();

        let issue = IssueType::FunctionNotFound{ unknown_fn };

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue

    }

    pub fn collect_type_out_of_range(
        &mut self,
        key : Key,
        expected_type : &Type,
    ) -> IssueType{
        self.lexer.error.has_error = true;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(key as usize);

        let ty_str = from_type_to_string(&expected_type);
        let issue = IssueType::LiteralOutOfRange{expected_type : ty_str.to_string()};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_mismatch_node_type_issue(
        &mut self,
        curr_node_ty : &Type,
        prev_ty   : &Type,
        start_key : usize
    ) -> IssueType{

        self.lexer.error.has_error = true;
        let curr_node_key = start_key;


        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(curr_node_key);


        let prev_node_type = match prev_ty{
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            _ => panic!("unknown node ty")
        }.to_string();

        let curr_node_type = match curr_node_ty{
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            _ => panic!("unknown node ty")
        }.to_string();

        let issue = IssueType::MismatchesNodeType {
            prev_node_type, curr_node_type
        };

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        let prev_node_ty = match prev_ty {
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => Token::Int,
            Type::f32 | Type::f64 => Token::Float,
            _ => panic!("unknown node ty")
        };

        let mut peek_idx = start_key + 1;
        let tokens = &self.lexer.tokenizer.tokens;
        while matches!(tokens[peek_idx], Token::Int | Token::Float | Token::String) || is_op(&tokens[peek_idx]){
            let peek = &tokens[peek_idx];

            if !is_op(&peek) && peek != &prev_node_ty{
                let(line,
                    rlt_col_start,
                    rlt_col_end,
                    _alt_col_start,
                    _alt_col_end,
                    span_id)
                    = self.get_data(peek_idx).clone();


                let prev_node_type = match prev_node_ty{
                    Token::Int => "integer",
                    Token::Float => "float",
                    Token::String => "String",
                    _ => panic!("unknown node ty")
                }.to_string();

                let curr_node_type = match peek{
                    Token::Int => "integer",
                    Token::Float => "float",
                    Token::String => "String",
                    _ => panic!("unknown node ty")
                }.to_string();

                let issue = IssueType::MismatchesNodeType {
                    prev_node_type, curr_node_type
                };

                self.lexer.error.msg_datas.push(
                    MsgData::new(
                        issue.clone(),
                        line,
                        rlt_col_start,
                        rlt_col_end,
                        span_id
                    )
                );
            }

            peek_idx += 1;
        }



        issue

    }



    pub fn get_data(&self, wrong_tok_key : usize) -> (usize, usize, usize, usize, usize, usize){

        let already_look_last_tok = self.current >= self.lexer.error.span_ids.len();
        //get range of sentence which has it



        let span_id = if already_look_last_tok{
            self.lexer.error.span_ids[wrong_tok_key - 1] as usize
        }else{
            self.lexer.error.span_ids[wrong_tok_key] as usize
        };

        let span_range = if already_look_last_tok{
            self.lexer.error.spans[span_id - 1]
        }else{
            self.lexer.error.spans[span_id]
        };

        let line = span_id + 1;
        //alt means absolute
        let alt_span_start = span_range.0 as usize;

        //get itself Absolute column
        let interval = if already_look_last_tok{
            &self.lexer.tokenizer.intervals[wrong_tok_key - 1]
        }else{
            &self.lexer.tokenizer.intervals[wrong_tok_key]
        };

        let alt_col_start = interval.start as usize;

        let alt_col_end = interval.end as usize;



        let rlt_col_start = alt_col_start - alt_span_start;
        let rlt_col_end = alt_col_end - alt_span_start;

        (line,  rlt_col_start, rlt_col_end, alt_col_start, alt_col_end, span_id)


    }




    pub fn out_of_this_stmt(&mut self){
        while !is_stmt(self.peek()){
            if matches!(self.peek(), Token::Rbrace) |matches!(self.peek(), Token::Scolon){
                self.eat();
                break;
            }

            self.eat();
        }


    }



}

fn is_stmt(tok : &Token) -> bool{
    matches!(tok,
        Token::EOF|
        Token::Rbrace|
        Token::Val|
        Token::Var |
        Token::If  |
        Token::Elif|
        Token::Else|
        Token::For |
        Token::While|
        Token::Ctn |
        Token::Brk |
        Token::Ret |
        Token::Print
    )


}


fn from_tok_to_string(tok : &Token) -> &str{
    match tok{
        Token::Val => "val",
        Token::Var => "var",
        Token::For => "for",
        Token::While => "while",
        Token::In => "in",
        Token::Ctn => "continue",
        Token::Brk => "break",
        Token::If => "if",
        Token::Elif => "elif",
        Token::Else => "else",
        Token::CmpNot => "not",
        Token::CmpAnd => "and",
        Token::CmpOr => "or",
        Token::Fn => "fn",
        Token::Ret => "return",
        Token::True => "true",
        Token::False => "false",
        Token::Print => "print",

        Token::Add => "+",
        Token::AddEq => "+=",
        Token::Sub => "-",
        Token::SubEq => "-=",
        Token::Mul => "*",
        Token::MulEq => "*=",
        Token::Div => "/",
        Token::DivEq => "/=",
        Token::Mod => "%",
        Token::ModEq => "%=",
        Token::Eq => "=",
        Token::EqEq => "==",
        Token::Lparen => "(",
        Token::Rparen => ")",
        Token::Lbrace => "{",
        Token::Rbrace => "}",
        Token::Lbracket => "[",
        Token::Rbracket => "]",
        Token::Colon => ":",
        Token::Scolon => ";",
        Token::Comma => ",",
        Token::Vertical => "|",
        Token::Ident => "an identifier",
        Token::Type => "a Type",
        _ => panic!("Expected Token : {:?}", tok)
    }
}

fn from_type_to_string(ty : &Type) -> &str{
    match ty{
        Type::void => "void",
        Type::i8 => "i8",
        Type::i16 => "i16",
        Type::i32 => "i32",
        Type::i64 => "i64",
        Type::i128 => "i128",
        Type::f32 => "f32",
        Type::f64 => "f64",
        Type::bool => "bool",
        Type::String => "String",
    }

}







#[test]
fn test_par_error(){
    let input =
        "fn main() = void {var x = 1; var y = x + 1;}  ".to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
    lexer.print_output();

    let mut parser = Parser::init(lexer);
    _ =parser.build_tree();
    dbg!(parser.scope.var_scope_stack);
    dbg!(parser.table.var_keys);
    if parser.lexer.error.has_error{
        parser.lexer.output_error();
    }

}



