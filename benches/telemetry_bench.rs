use criterion::{criterion_group, Criterion, criterion_main};
use comp::lexer::center::lex::Lexer;
use comp::parser::center::par::Parser;

fn bench_joyc(c : &mut Criterion){
    let input = "
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn check_range(val: i32) -> i32 {
        if val < 10 {
            return 1;
        } elif val < 50 {
            return 2;
        } else {
            return 3;
        }
        return 4;
    }

    fn factorial(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        return n * factorial(n - 1);
    }

    fn main() {
        let x = add(15, 35);
        let status = check_range(x);
        let fact = factorial(5);

        let total = 0;
        for i in [1 -> 10 | 1]{
            break;
        }

        return;
    }
    ".to_string();


    c.bench_function("joyc",|b| {
        b.iter(|| {
            let mut lexer = Lexer::init(input.to_string(), "main.joy".to_string());
            lexer.tokenize();
            let mut parser = Parser::init(lexer);
            parser.build_tree();
        })
    });

}
criterion_group!(benches, bench_joyc);
criterion_main!(benches);