// https://leetcode.com/problems/basic-calculator-ii/
// 227. Basic Calculator II
pub struct Solution;
#[derive(Copy, Clone)]
enum Ele {
    Number(i32),
    Op(u8),
}
const LVL_PLUS: usize = 0;
const LVL_TIMES: usize = 1;
impl Solution {
    fn calc(stack: &mut Vec<Ele>, lvl: usize) {
        if stack.len() < 3 {
            return;
        }
        while stack.len() > 2 {
            let l = stack.len() - 1;
            if let Ele::Number(b) = stack[l] {
                if let Ele::Op(c) = stack[l - 1] {
                    if c == b'-' || c == b'+' {
                        if lvl == LVL_TIMES {
                            break;
                        }
                        if let Ele::Number(a) = stack[l - 2] {
                            if c == b'-' {
                                stack[l - 2] = Ele::Number(a - b);
                            } else {
                                stack[l - 2] = Ele::Number(a + b);
                            }
                            stack.truncate(l - 1);
                        }
                    } else if c == b'*' || c == b'/' {
                        if let Ele::Number(a) = stack[l - 2] {
                            if c == b'*' {
                                stack[l - 2] = Ele::Number(a * b);
                            } else {
                                stack[l - 2] = Ele::Number(a / b);
                            }
                            stack.truncate(l - 1);
                        }
                    }
                }
            }
        }
    }
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<Ele> = Vec::new();
        for &c in s.as_bytes() {
            match c {
                b'-' | b'+' => {
                    Self::calc(&mut stack, LVL_PLUS);
                    stack.push(Ele::Op(c));
                }
                b'*' | b'/' => {
                    Self::calc(&mut stack, LVL_TIMES);
                    stack.push(Ele::Op(c));
                }
                b'0'..=b'9' => {
                    if let Some(Ele::Number(n)) = stack.last_mut() {
                        *n = *n * 10 + (c - b'0') as i32;
                    } else {
                        stack.push(Ele::Number((c - b'0') as i32));
                    }
                }
                b' ' => (),
                _ => {
                    panic!("error")
                }
            }
        }
        Self::calc(&mut stack, LVL_PLUS);
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
        assert_eq!(Solution::calculate(String::from("3+2*2")), 7);
        assert_eq!(Solution::calculate(String::from(" 3/2 ")), 1);
        assert_eq!(Solution::calculate(String::from(" 3+5 / 2 ")), 5);
    }
}
