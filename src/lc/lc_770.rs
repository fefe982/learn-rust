// https://leetcode.com/problems/basic-calculator-iv/
// 770. Basic Calculator IV
pub struct Solution;
#[derive(Debug)]
enum Token {
    LBracket,
    RBracket,
    Ident(String),
    Number(i32),
    Op(char),
}
#[derive(Debug, PartialEq, Eq)]
enum State {
    Ident,
    Number,
    Unkown,
}

type Term = std::collections::BTreeMap<String, i32>;
type Expr = std::collections::HashMap<Term, i32>;
impl Solution {
    fn push_token(tokens: &mut Vec<Token>, state: &mut State, buffer: &str, number: i32) {
        match *state {
            State::Number => tokens.push(Token::Number(number)),
            State::Ident => tokens.push(Token::Ident(buffer.to_owned())),
            _ => {}
        };
        *state = State::Unkown;
    }
    fn tokenize(expression: String) -> Vec<Token> {
        let mut tokens = vec![];
        let mut state = State::Unkown;
        let mut buffer = String::new();
        let mut number = 0;

        for c in expression.chars() {
            match c {
                ' ' => Self::push_token(&mut tokens, &mut state, &buffer, number),
                '(' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number);
                    tokens.push(Token::LBracket);
                }
                ')' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number);
                    tokens.push(Token::RBracket);
                }
                '-' | '+' | '*' => {
                    Self::push_token(&mut tokens, &mut state, &buffer, number);
                    tokens.push(Token::Op(c));
                }
                '0'..='9' => match state {
                    State::Ident => buffer.push(c),
                    State::Number => number = number * 10 + (c as i32 - '0' as i32),
                    _ => {
                        Self::push_token(&mut tokens, &mut state, &buffer, number);
                        state = State::Number;
                        number = c as i32 - '0' as i32;
                    }
                },
                'a'..='z' | 'A'..='Z' => {
                    if State::Ident != state {
                        Self::push_token(&mut tokens, &mut state, &buffer, number);
                        state = State::Ident;
                        buffer.clear();
                    }
                    buffer.push(c);
                }
                _ => unreachable!(),
            }
        }
        Self::push_token(&mut tokens, &mut state, &buffer, number);
        tokens
    }
    fn filter(expr: Expr) -> Expr {
        expr.into_iter().filter(|e| e.1 != 0).collect::<Expr>()
    }
    fn parse_term(tokens: &[Token], var_map: &std::collections::HashMap<String, i32>) -> (usize, Expr) {
        let mut expr = Expr::new();
        let mut idx = 0;
        if idx < tokens.len() {
            match &tokens[0] {
                Token::LBracket => {
                    let (consumed, next) = Solution::parse_plus(&tokens[1..], var_map);
                    idx += consumed + 1;
                    if let Token::RBracket = tokens[idx] {
                    } else {
                        unreachable!()
                    }
                    idx += 1;
                    expr = next;
                }
                Token::Ident(ident) => {
                    if let Some(v) = var_map.get(ident) {
                        expr.insert(Term::new(), *v);
                    } else {
                        let mut t = Term::new();
                        t.insert(ident.clone(), 1);
                        expr.insert(t, 1);
                    }
                    idx += 1;
                }
                Token::Number(n) => {
                    expr.insert(Term::new(), *n);
                    idx += 1;
                }
                _ => unreachable!(),
            }
        }
        (idx, Solution::filter(expr))
    }
    fn parse_mul(tokens: &[Token], var_map: &std::collections::HashMap<String, i32>) -> (usize, Expr) {
        let mut expr = Expr::new();
        expr.insert(Term::new(), 1);
        let mut idx = 0;
        while idx < tokens.len() {
            let (consumed, next) = Self::parse_term(&tokens[idx..], var_map);
            let mut ne = Expr::new();
            for (t1, c1) in expr {
                for (t2, c2) in next.iter() {
                    let mut t = t1.clone();
                    for (var, p) in t2 {
                        *t.entry(var.clone()).or_default() += *p;
                    }
                    *ne.entry(t).or_default() += c1 * c2;
                }
            }
            expr = ne;
            idx += consumed;
            if idx >= tokens.len() {
                break;
            }
            if let Token::Op('*') = tokens[idx] {
                idx += 1;
            } else {
                break;
            }
        }
        (idx, Solution::filter(expr))
    }
    fn parse_plus(tokens: &[Token], var_map: &std::collections::HashMap<String, i32>) -> (usize, Expr) {
        let mut expr = Expr::new();
        let mut idx = 0;
        let mut lastop = 1;
        while idx < tokens.len() {
            let (consumed, next) = Self::parse_mul(&tokens[idx..], var_map);
            for (term, coef) in next {
                *expr.entry(term).or_default() += lastop * coef;
            }
            idx += consumed;
            if idx >= tokens.len() {
                break;
            }
            if let Token::Op(c) = tokens[idx] {
                if c == '+' {
                    lastop = 1;
                    idx += 1;
                } else if c == '-' {
                    lastop = -1;
                    idx += 1;
                } else {
                    unreachable!()
                }
            } else {
                break;
            }
        }
        (idx, Solution::filter(expr))
    }
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        let var_map = evalvars
            .into_iter()
            .zip(evalints)
            .collect::<std::collections::HashMap<_, _>>();
        let tokens = Self::tokenize(expression);
        let (_, result) = Solution::parse_plus(&tokens, &var_map);
        let mut terms = result
            .into_iter()
            .map(|(term, coef)| {
                let mut t = vec![];
                for (var, count) in term.into_iter() {
                    for _ in 0..count {
                        t.push(var.clone());
                    }
                }
                t.sort_unstable();
                (t, coef)
            })
            .collect::<Vec<_>>();
        terms.sort_by(|a, b| match a.0.len().cmp(&b.0.len()) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
        });
        terms
            .into_iter()
            .map(|(t, c)| {
                let mut s = c.to_string();
                for var in t.into_iter() {
                    s.push('*');
                    s.push_str(&var)
                }
                s
            })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_basic_calculator_iv() {
        assert_eq!(
            Solution::basic_calculator_iv("e + 8 - a + 5".to_string(), vec_str!["e"], vec![1]),
            vec_str!["-1*a", "14"]
        );
        assert_eq!(
            Solution::basic_calculator_iv(
                "e - 8 + temperature - pressure".to_string(),
                vec_str!["e", "temperature"],
                vec![1, 12]
            ),
            vec_str!["-1*pressure", "5"]
        );
        assert_eq!(
            Solution::basic_calculator_iv("(e + 8) * (e - 8)".to_string(), vec_str![], vec![]),
            vec_str!["1*e*e", "-64"]
        );
    }
}
