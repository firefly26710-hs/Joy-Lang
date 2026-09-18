use comp::lex::lexer::Lexer;
use comp::par::parser::Parser;
use criterion::{criterion_group, criterion_main, Criterion};
fn bench_joyc(c : &mut Criterion){
    let input = "
    fn fib(a : i32){
           return;
    }

    fn main(){
         let n = 5;
         fib(n);
    }



";


    c.bench_function("joyc",|b| {
        b.iter(|| {
            let mut lexer = Lexer::init(input.to_string());
            lexer.tokenize();
            let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.intervals);
            parser.build_tree();
        })
    });

}
criterion_group!(benches, bench_joyc);
criterion_main!(benches);