pub struct Parser {
    lookahead: char,
    input: String,
    output: Vec<Algebra>
}

#[derive(Debug, Clone)]
pub enum Algebra {
    Plus,
    Minus,
    Div,
    Mult,
    Number(i32)
}

impl Parser {
    pub fn output(&mut self) -> &Vec<Algebra> {
        &self.output
    }

    pub fn parser(mut input: String) -> Self {
        let first_lookahead = input.remove(0);
        Parser{lookahead: first_lookahead, input: input, output: Vec::new()}
    }

    pub fn expr(&mut self) {
        self.term();
        loop {
            if self.lookahead == '+' {
                self.parse_match('+');
                self.term();
                self.output.push(Algebra::Plus);
            } else if self.lookahead == '-' {
                self.parse_match('-');
                self.term();
                self.output.push(Algebra::Minus);
            } else {
                return
            }
        }
    }

    pub fn term(&mut self) {
        self.factor();
        loop {
            if self.lookahead == '*' {
                self.parse_match('*');
                self.factor();
                self.output.push(Algebra::Mult);
            } else if self.lookahead == '/' {
                self.parse_match('/');
                self.term();
                self.output.push(Algebra::Div);
            } else {
                return
            }
        }
    }

    pub fn factor(&mut self) {
        if self.lookahead == '(' {
            self.parse_match('(');
            self.expr();
            self.parse_match(')');
        } else if self.lookahead.is_digit(10) {
            // Scan forward for the entire number
            self.output.push(Algebra::Number(self.lookahead.to_digit(10).unwrap() as i32));
            self.parse_match(self.lookahead);
        }
        // else if self.lookahead.is_alphabetic() {
        //     // Scan forward for the entire id
        //     self.output.push(self.lookahead);
        //     self.parse_match(self.lookahead);
        // }
    }

    pub fn parse_match(&mut self, t: char) {
        if self.lookahead == t && self.input != "" {
            self.lookahead = self.input.remove(0);
        }
    }
}

#[test]
fn test_expr() {
    let input: String = String::from("1+4/2+3");
    let mut m = Parser::parser(input);
    m.expr();
    dbg!(m.output());
}
