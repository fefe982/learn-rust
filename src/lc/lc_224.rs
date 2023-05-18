// https://leetcode.com/problems/basic-calculator/
// 224. Basic Calculator
pub struct Solution;
#[derive(Copy, Clone)]
enum Ele {
    Number(i32),
    Op(u8),
}
impl Solution {
    fn add_default_num(stack: &mut Vec<Ele>) {
        if stack.len() == 0 {
            stack.push(Ele::Number(0));
        } else if let Ele::Op(_) = stack[stack.len() - 1] {
            stack.push(Ele::Number(0));
        }
    }
    fn calc(stack: &mut Vec<Ele>) {
        if stack.len() < 3 {
            return;
        }
        let l = stack.len() - 1;
        if let Ele::Number(b) = stack[l] {
            if let Ele::Op(c) = stack[l - 1] {
                if c == b'-' || c == b'+' {
                    if let Ele::Number(a) = stack[l - 2] {
                        if c == b'-' {
                            stack[l - 2] = Ele::Number(a - b);
                        } else {
                            stack[l - 2] = Ele::Number(a + b);
                        }
                        stack.resize(l - 1, Ele::Number(0));
                    }
                }
            }
        }
    }
    fn pop(stack: &mut Vec<Ele>) {
        if stack.len() < 2 {
            return;
        }
        let l = stack.len() - 1;
        if let Ele::Number(b) = stack[l] {
            if let Ele::Op(c) = stack[l - 1] {
                if c == b'(' {
                    stack[l - 1] = Ele::Number(b);
                    stack.resize(l, Ele::Number(0));
                    Self::calc(stack);
                }
            }
        }
    }
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<Ele> = Vec::new();
        for &c in s.as_bytes() {
            match c {
                b'-' | b'+' => {
                    Self::add_default_num(&mut stack);
                    Self::calc(&mut stack);
                    stack.push(Ele::Op(c));
                }
                b'0'..=b'9' => {
                    Self::add_default_num(&mut stack);
                    let l = stack.len() - 1;
                    if let Ele::Number(n) = stack[l] {
                        stack[l] = Ele::Number(n * 10 + (c - b'0') as i32);
                    }
                }
                b'(' => {
                    Self::calc(&mut stack);
                    stack.push(Ele::Op(c));
                }
                b')' => {
                    Self::calc(&mut stack);
                    Self::pop(&mut stack);
                }
                b' ' => (),
                _ => {
                    panic!("error")
                }
            }
        }
        Self::calc(&mut stack);
        if let Ele::Number(n) = stack[0] {
            n
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate() {
        assert_eq!(Solution::calculate(String::from("1 + 1")), 2);
        assert_eq!(Solution::calculate(String::from(" 2-1 + 2 ")), 3);
        assert_eq!(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
    }
}
