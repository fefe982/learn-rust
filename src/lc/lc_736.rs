// https://leetcode.com/problems/parse-lisp-expression/
// 736. Parse Lisp Expression
pub struct Solution;
#[derive(Debug)]
enum Token {
    LBracket,
    RBracket,
    Ident(String),
    Number(i32),
}
#[derive(Debug, PartialEq, Eq)]
enum State {
    Ident,
    Number,
    Unkown,
}
#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Let,
    Add,
    Mult,
}
#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    ExprBegin,
    LetIdent,
    LetExpr,
    LetEnd,
    Expr,
}
impl Solution {
    fn push_token(tokens: &mut Vec<Token>, state: &mut State, buffer: &str, number: i32, sign: i32) {
        match *state {
            State::Number => tokens.push(Token::Number(number * sign)),
            State::Ident => tokens.push(Token::Ident(buffer.to_owned())),
            _ => {}
        };
        *state = State::Unkown;
    }
    fn tokenize(expression: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut state = State::Unkown;
        let mut buffer = String::new();
        let mut number = 0;
        let mut sign = 1;

        for c in expression.chars() {
            match c {
                ' ' => Self::push_token(&mut tokens, &mut state, &buffer, number, sign),
                '(' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number, sign);
                    tokens.push(Token::LBracket);
                }
                ')' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number, sign);
                    tokens.push(Token::RBracket);
                }
                '-' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number, sign);
                    state = State::Number;
                    number = 0;
                    sign = -1;
                }
                '0'..='9' => match state {
                    State::Ident => buffer.push(c),
                    State::Number => number = number * 10 + (c as i32 - '0' as i32),
                    _ => {
                        Self::push_token(&mut tokens, &mut state, &buffer, number, sign);
                        state = State::Number;
                        number = c as i32 - '0' as i32;
                        sign = 1;
                    }
                },
                'a'..='z' | 'A'..='Z' => {
                    if State::Ident != state {
                        Self::push_token(&mut tokens, &mut state, &buffer, number, sign);
                        state = State::Ident;
                        buffer.clear();
                    }
                    buffer.push(c);
                }
                _ => unreachable!(),
            }
        }
        tokens
    }
    fn get_val(stack_var: &Vec<std::collections::HashMap<String, i32>>, ident: String) -> i32 {
        for i in stack_var.iter().rev() {
            if let Some(val) = i.get(&ident) {
                return *val;
            }
        }
        unreachable!()
    }
    pub fn evaluate(expression: String) -> i32 {
        let tokens = Self::tokenize(expression);
        let mut stack_val = Vec::new();
        let mut stack_op = Vec::new();
        let mut stack_var = Vec::<std::collections::HashMap<String, i32>>::new();
        let mut stack_state = vec![ParseState::Expr];
        let mut stack_let_ident = Vec::<String>::new();
        for token in tokens {
            match token {
                Token::LBracket => {
                    match *stack_state.last().unwrap() {
                        ParseState::LetExpr => *stack_state.last_mut().unwrap() = ParseState::LetIdent,
                        ParseState::LetIdent => *stack_state.last_mut().unwrap() = ParseState::LetEnd,
                        ParseState::Expr => {}
                        _ => unreachable!(),
                    }
                    stack_state.push(ParseState::ExprBegin);
                }
                Token::RBracket => {
                    match stack_op.pop().unwrap() {
                        Operator::Add => {
                            let a = stack_val.pop().unwrap();
                            let b = stack_val.pop().unwrap();
                            stack_val.push(a + b);
                        }
                        Operator::Mult => {
                            let a = stack_val.pop().unwrap();
                            let b = stack_val.pop().unwrap();
                            stack_val.push(a * b);
                        }
                        Operator::Let => {
                            match stack_state.last().unwrap() {
                                ParseState::LetExpr => {
                                    stack_val.push(Self::get_val(&stack_var, stack_let_ident.last().unwrap().clone()));
                                }
                                ParseState::LetEnd => {}
                                _ => unreachable!(),
                            };
                            stack_var.pop();
                            stack_let_ident.pop();
                        }
                    };
                    stack_state.pop();
                    match *stack_state.last().unwrap() {
                        ParseState::LetIdent => {
                            stack_var
                                .last_mut()
                                .unwrap()
                                .insert(stack_let_ident.last().unwrap().clone(), stack_val.pop().unwrap());
                        }
                        ParseState::Expr | ParseState::LetEnd => {}
                        _ => unreachable!(),
                    }
                }
                Token::Ident(s) => {
                    let last_state = stack_state.last_mut().unwrap();
                    match *last_state {
                        ParseState::ExprBegin => {
                            if s == "let" {
                                stack_op.push(Operator::Let);
                                stack_var.push(std::collections::HashMap::new());
                                stack_let_ident.push(String::new());
                                *last_state = ParseState::LetIdent;
                            } else if s == "mult" {
                                stack_op.push(Operator::Mult);
                                *last_state = ParseState::Expr;
                            } else if s == "add" {
                                stack_op.push(Operator::Add);
                                *last_state = ParseState::Expr;
                            } else {
                                unreachable!()
                            }
                        }
                        ParseState::LetIdent => {
                            *stack_let_ident.last_mut().unwrap() = s;
                            *last_state = ParseState::LetExpr;
                        }
                        ParseState::Expr => {
                            stack_val.push(Self::get_val(&stack_var, s));
                        }
                        ParseState::LetExpr => {
                            let v = Self::get_val(&stack_var, s);
                            stack_var
                                .last_mut()
                                .unwrap()
                                .insert(stack_let_ident.last().unwrap().clone(), v);
                            *last_state = ParseState::LetIdent;
                        }
                        _ => unreachable!(),
                    }
                }
                Token::Number(n) => {
                    let last_state = stack_state.last_mut().unwrap();
                    match *last_state {
                        ParseState::LetExpr => {
                            stack_var
                                .last_mut()
                                .unwrap()
                                .insert(stack_let_ident.last().unwrap().clone(), n);
                            *last_state = ParseState::LetIdent;
                        }
                        ParseState::LetIdent => {
                            stack_val.push(n);
                            *last_state = ParseState::LetEnd;
                        }
                        ParseState::Expr => {
                            stack_val.push(n);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        stack_val.pop().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        assert_eq!(Solution::evaluate("(let y 2 x (let q 2 z y 2) x)".to_string()), 2);
        assert_eq!(Solution::evaluate("(let x 7 -12)".to_string()), -12);
        assert_eq!(Solution::evaluate("(let a1 3 b2 (add a1 1) b2)".to_string()), 4);
        assert_eq!(
            Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
            14
        );
        assert_eq!(Solution::evaluate("(let x 3 x 2 x)".to_string()), 2);
        assert_eq!(Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()), 5);
    }
}
