use crate::lexer::center::lex::Lexer;
use crate::parser::diagnostic::source::issue::ScopeIssue;
use crate::parser::diagnostic::source::issue::{ExprIssue, TokenIssue};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;


impl Parser{
    pub fn unexpected_token_issue(
        &mut self,
        correct_token : &Token,
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let wrong_string = self.lexer.tokenizer.input[start..end].to_string();
        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue = ParserIssue::Token(TokenIssue::UnExpectedToken {wrong_string, correct_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn unclosed_delimiter_token_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::Token(TokenIssue::UnclosedDelimiter);

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn unexpected_expression_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let wrong_string = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Expr(ExprIssue::UnExpectedExpression{wrong_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                span_id + 1,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn mismatch_type_issue(
        &mut self,
        expr_start_key : Key,
        expected_type : &Type,
        actual_type : &Type
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let actual_type_key = expr_start_key as usize;

        let span_id = self.lexer.error.span_ids[actual_type_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[actual_type_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let expected_type = from_type_to_string(expected_type).to_string();
        let actual_type = from_type_to_string(actual_type).to_string();

        let issue = ParserIssue::Expr(ExprIssue::MismatchesType {expected_type, actual_type});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                span_id + 1,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn variable_not_in_scope_issue(&mut self, wrong_tok_key: Key ) -> ParserIssue {
        self.lexer.error.has_error = true;
        let wrong_tok_key = wrong_tok_key as usize;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let unknown_var_string = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::VariableNotInScope { unknown_var_string });

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }

    pub fn cannot_found_function_issue(&mut self, key: Key) -> ParserIssue {
        self.lexer.error.has_error = true;
        let unknown_func_key = key as usize;

        let span_id = self.lexer.error.span_ids[unknown_func_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[unknown_func_key];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_fn = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::FunctionNotFound { unknown_fn_string: unknown_fn });

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }

    pub fn type_out_of_range(
        &mut self,
        key : Key,
        expected_type : &Type,
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_literal_key = key as usize;
        let span_id = self.lexer.error.span_ids[wrong_literal_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[span_id];
        let start = interval.start as usize;
        let end = interval.end as usize;


        let expected_type_string = from_type_to_string(&expected_type).to_string();
        let issue = ParserIssue::Expr(ExprIssue::LiteralOutOfRange { expected_type_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn comparison_op_chained_issue(&mut self, key : Key) -> ParserIssue {
        self.lexer.error.has_error = true;

        dbg!(key);
        let second_comp_op_key = key as usize;

        let span_id = self.lexer.error.span_ids[second_comp_op_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[second_comp_op_key];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::Expr(ExprIssue::ComparisonOpChained);

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn mismatch_node_type_issue(
        &mut self,
        left_type: &Type,
        right_type: &Type,
        right_expr_key: usize
    ) -> ParserIssue {

        self.lexer.error.has_error = true;
        let right_expr_key = right_expr_key;

        let span_id = self.lexer.error.span_ids[right_expr_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[right_expr_key];
        let start = interval.start as usize;
        let end = interval.end as usize;



        let left_node_type = match left_type {
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            Type::bool => "bool",
            _ => panic!("unknown node ty")
        }.to_string();

        let right_node_type = match right_type {
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            Type::bool => "bool",
            _ => panic!("unknown node ty")
        }.to_string();

        let issue = ParserIssue::Expr(ExprIssue::MismatchesNodeType {
            left_node_type, right_node_type
        });

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }

    pub fn outside_of_a_loop_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let kw_key = self.current;

        let span_id = self.lexer.error.span_ids[kw_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[kw_key];
        let start = interval.start as usize;
        let end = interval.end as usize;


        let kw = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::OutOfLoop {kw});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }





    pub fn out_of_this_stmt(&mut self){
        self.eat();
        while !is_stmt(self.peek()){
            if matches!(self.peek(), Token::Rbrace) | matches!(self.peek(), Token::Scolon){
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
        "fn main() = void {var x = 5 > 1 < 3;}".to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
    lexer.print_output();
    let mut parser = Parser::init(lexer);
    _ =parser.build_tree();
    parser.print_node();
    dbg!(parser.scope.var_scope_stack);
    dbg!(parser.table.var_keys);
    dbg!(parser.table.var_types);


}



