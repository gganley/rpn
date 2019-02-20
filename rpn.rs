use rpn;

fn solve(list: Vec<rpn::Algebra>) -> i32 {
    let mut results: Vec<i32> = Vec::new();

    for element in list {
        match element {
            rpn::Algebra::Plus => {
                let x1 = results.pop().unwrap();
                let x2 = results.pop().unwrap();
                results.push(x2 + x1);
            },
            rpn::Algebra::Minus => {
                let x1 = results.pop().unwrap();
                let x2 = results.pop().unwrap();
                results.push(x2 - x1);
            },
            rpn::Algebra::Div => {
                let x1 = results.pop().unwrap();
                let x2 = results.pop().unwrap();
                results.push(x2 / x1);
            },
            rpn::Algebra::Mult => {
                let x1 = results.pop().unwrap();
                let x2 = results.pop().unwrap();
                results.push(x2 * x1);
            },
            rpn::Algebra::Number(a) => {results.push(a);}
        }
    }
    results.pop().unwrap()
}

fn main() {
    let input: String = String::from("1+4/2+3");
    let mut m = rpn::Parser::parser(input);
    m.expr();
    dbg!(m.output());
    dbg!(solve(m.output().to_vec()));
}
